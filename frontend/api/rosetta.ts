import { LEDGER_CANISTER_ID, MILI_PER_SECOND, NET_ID, ROSETTA_URL } from "@/api/constants/ic";
import { getICPPrice } from "@/api/token";

export interface InferredTransaction {
    hash: string;
    timestamp: bigint;
    type: string;
    details?: { [key: string]: any };
    caller: string;
}

export interface GetTransactionsResponse {
    total: number;
    transactions: InferredTransaction[];
}

export const getICPTransactions = async (
    accountId: string
): Promise<GetTransactionsResponse> => {
    const response = await fetch(`${ROSETTA_URL}/search/transactions`, {
        method: 'POST',
        body: JSON.stringify({
            network_identifier: NET_ID,
            account_identifier: {
                address: accountId,
            },
        }),
        headers: {
            'Content-Type': 'application/json',
            Accept: '*/*',
        },
    });
    if (!response.ok)
        throw Error("error for rosetta api"+ response.statusText);
    const { transactions, total_count } = await response.json();
    console.log("rosetta api:",transactions)
    // 使用 Promise.all 来并行获取价格
    const transactionsInfoPromises = transactions.map(async ({ transaction }) => {
        const formattedTransaction = await formatIcpTransaccion(accountId, transaction);
        return formattedTransaction;
    });

    // 等待所有价格获取完成
    const transactionsInfo = await Promise.all(transactionsInfoPromises);
    console.log("transactionsInfo",transactionsInfo)
    return {
        total: total_count,
        transactions: transactionsInfo,
    };
};

interface Operation {
    account: {
        address: string;
    };
    amount: {
        value: string;
        currency: {
            symbol: string;
            decimals: number;
        };
    };
    status: 'COMPLETED' | 'REVERTED' | 'PENDING';
    type: 'TRANSACTION' | 'FEE';
}

interface RosettaTransaction {
    metadata: {
        block_height: number;
        memo: number;
        timestamp: number;
        lockTime: number;
    };
    operations: Operation[];
    transaction_identifier: { hash: string };
}

export const formatIcpTransaccion = async (
    accountId: string,
    rosettaTransaction: RosettaTransaction
): Promise<InferredTransaction>  => {
    const {
        operations,
        metadata: { timestamp },
        transaction_identifier: { hash },
    } = rosettaTransaction;
    const transaction: any = { details: { status: 'COMPLETED', fee: {} } };
    const timestampNormal =  timestamp / MILI_PER_SECOND; //处理时间戳为正常格式
    const price = await getICPPrice(timestampNormal); // 使用 await 获取价格
    operations.forEach(operation => {
        const value = BigInt(operation.amount.value);
        const amount = value.toString();
        if (operation.type === 'FEE') {
            transaction.details.fee.amount = amount;
            transaction.details.fee.currency = operation.amount.currency;
            return;
        }

        if (value >= 0) transaction.details.to = operation.account.address;
        if (value <= 0) transaction.details.from = operation.account.address;

        if (
            transaction.details.status === 'COMPLETED' &&
            operation.status !== 'COMPLETED'
        )
            transaction.details.status = operation.status;

        transaction.type =
            transaction.details.to === accountId ? 'RECEIVE' : 'SEND';
        transaction.details.amount = amount;
        transaction.details.price = price; // 设置价格为获取的价格
        transaction.details.currency = operation.amount.currency;
        transaction.details.canisterId = LEDGER_CANISTER_ID;
    });
    return {
        ...transaction,
        caller: transaction.details.from,
        hash,
        timestamp: timestampNormal,
    } as InferredTransaction;
};
