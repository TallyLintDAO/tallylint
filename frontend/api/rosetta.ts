import { LEDGER_CANISTER_ID, MILI_PER_SECOND, NET_ID, ROSETTA_URL } from "@/api/constants/ic";
import { getICPPrice } from "@/api/token";
import { currencyCalculate } from "@/utils/common";
import { showMessageError } from "@/utils/message";

const radixNumber = 4;//保留4位小数

export interface InferredTransaction {
    hash: string;
    timestamp: bigint;
    type: string;
    details: {
        status: string;
        fee: {
            amount: number;
            currency: {
                decimals: number;
                symbol: string;
            };
        };
        to?: string;
        from?: string;
        amount: number;
        price: number; // 添加这两个属性的定义
        currency: {
            decimals: number;
            symbol: string;
        };
        canisterId: string;
        cost: number;
        profit: number;
        value: number;
    };
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
    if (!response.ok) {
        showMessageError('Can not connect ICP rosetta api')
        throw Error("error for rosetta api" + response.statusText);
    }
    const {transactions, total_count} = await response.json();
    console.log("rosetta api:", transactions)
    const transactionsInfo: InferredTransaction[] = [];
    //由于是时间最新的排前，所以要倒序数组，以实现先入先出的税务计算方式
    transactions.reverse();
    for (const {transaction} of transactions) {
        const formattedTransaction = await formatIcpTransaccion(accountId, transaction);
        transactionsInfo.push(formattedTransaction);
    }
    //将数组恢复正常。
    transactionsInfo.reverse();
    console.log("transactionsInfo", transactionsInfo)
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
): Promise<InferredTransaction> => {
    const {
        operations,
        metadata: {timestamp},
        transaction_identifier: {hash},
    } = rosettaTransaction;
    const transaction: any = {details: {status: 'COMPLETED', fee: {}}};
    const timestampNormal = timestamp / MILI_PER_SECOND; //处理时间戳为正常格式
    const price = await getICPPrice(timestampNormal); // 使用 await 获取价格
    operations.forEach(operation => {
        const value = BigInt(operation.amount.value);
        const amount = value.toString();
        if (operation.type === 'FEE') {
            //直接输出真实的数量，不再使用浮点数
            transaction.details.fee.amount = currencyCalculate(amount, operation.amount.currency.decimals);
            transaction.details.fee.currency = operation.amount.currency;
            return;
        }

        if (value >= 0) transaction.details.to = operation.account.address;
        if (value <= 0) transaction.details.from = operation.account.address;

        if (
            transaction.details.status === 'COMPLETED' &&
            operation.status !== 'COMPLETED'
        ) transaction.details.status = operation.status;

        transaction.type =
            transaction.details.to === accountId ? 'RECEIVE' : 'SEND';

        //直接输出真实的数量，不再使用浮点数
        transaction.details.amount = currencyCalculate(amount, operation.amount.currency.decimals);
        transaction.details.price = price; // 设置价格为获取的价格
        transaction.details.value = parseFloat(
            (transaction.details.amount * transaction.details.price).toFixed(radixNumber)); //计算总价值
        transaction.details.currency = operation.amount.currency;
        transaction.details.canisterId = LEDGER_CANISTER_ID;

        //先入先出的成本计算法，以IC的精度，建议保留4位小数
        const cost = calculateCost(transaction);
        transaction.details.cost = parseFloat(cost.toFixed(radixNumber));
        if (transaction.type === 'RECEIVE') {
            transaction.details.profit = 0;
        } else if (transaction.type === 'SEND') {
            // const factor = 10 ** radixNumber; //进位10的n次方，扩大倍数将其变成整数，再在计算完成后除以倍数换回小数点
            //TODO 本意是计算精度更准确，但有点bug，先注释了，用简单粗暴的
            // transaction.details.profit =
            //     (transaction.details.value * factor
            //         - transaction.details.cost * factor) / factor;
            transaction.details.profit = (transaction.details.value - transaction.details.cost).toFixed(radixNumber);
        }
    });
    return {
        ...transaction,
        caller: transaction.details.from,
        hash,
        timestamp: timestampNormal,
    } as InferredTransaction;
};

const purchaseQueue: any[] = [];

// 计算FIFO成本
const calculateCost = (transaction: InferredTransaction): number => {
    if (transaction.type === 'RECEIVE') {
        // 处理接收交易，保存价格和数量。
        const {price, amount} = transaction.details
        purchaseQueue.push({price, amount})
        return 0
    } else if (transaction.type === 'SEND') {
        let cost = 0;
        let sendAmount = transaction.details.amount; // 存储本次交易发送的代币数量

        while (sendAmount > 0 && purchaseQueue.length > 0) {
            // 从最早购买的交易开始卖出
            const earliestPurchase = purchaseQueue[0];

            if (earliestPurchase.amount <= sendAmount) {
                // 如果购买数量小于等于发送数量，则完全卖出该购买交易
                cost += earliestPurchase.price * earliestPurchase.amount;
                sendAmount -= earliestPurchase.amount;
                purchaseQueue.shift(); // 从队列中移除已卖出的交易
            } else {
                // 如果购买数量大于发送数量，则部分卖出该购买交易
                cost += earliestPurchase.price * sendAmount;
                earliestPurchase.amount -= sendAmount;
                sendAmount = 0;
            }
        }

        return cost;
    } else {
        return 0
    }
};
