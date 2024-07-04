struct Transaction {
  action: String,
  quantity: i32,
  price: f64,
  profit: Option<f64>,
}

struct Product {
  transactions: Vec<Transaction>,
}

impl Product {
  fn new() -> Self {
    Product {
      transactions: Vec::new(),
    }
  }

  fn buy(&mut self, quantity: i32, price: f64) {
    self.transactions.push(Transaction {
      action: "sell".to_string(),
      quantity,
      price,
      profit: None,
    });
  }

  fn sell_lifo(&mut self, quantity: i32, price: f64) -> f64 {
    let mut remaining = quantity;
    let mut gain_or_loss = 0.0;

    while remaining > 0 && !self.transactions.is_empty() {
      let transaction = &mut self.transactions.last_mut().unwrap();
      if transaction.quantity <= remaining {
        gain_or_loss +=
          (price - transaction.price) * (transaction.quantity as f64);
        remaining -= transaction.quantity;
        self.transactions.pop();
      } else {
        gain_or_loss += (price - transaction.price) * (remaining as f64);
        transaction.quantity -= remaining;
        remaining = 0;
      }
    }

    gain_or_loss
  }
  fn sell_fifo(&mut self, quantity: i32, price: f64) -> f64 {
    let mut remaining = quantity;
    let mut gain_or_loss = 0.0;

    while remaining > 0 && !self.transactions.is_empty() {
      let transaction = &mut self.transactions[0];
      if transaction.quantity <= remaining {
        gain_or_loss +=
          (price - transaction.price) * (transaction.quantity as f64);
        remaining -= transaction.quantity;
        self.transactions.remove(0);
      } else {
        gain_or_loss += (price - transaction.price) * (remaining as f64);
        transaction.quantity -= remaining;
        remaining = 0;
      }
    }

    gain_or_loss
  }
}
fn sell_hifo(&mut self, quantity: f64, price: f64) -> f64 {
  let mut remaining = quantity;
  let mut gain_or_loss = 0.0;

  while remaining > 0.0 && !self.transactions.is_empty() {
      // Find the transaction with the highest price
      let max_price_index = self.transactions.iter().enumerate()
          .max_by(|(_, a), (_, b)| a.price.partial_cmp(&b.price).unwrap())
          .map(|(index, _)| index)
          .unwrap();

      let transaction = &mut self.transactions[max_price_index];
      if transaction.quantity <= remaining {
          gain_or_loss += (price - transaction.price) * transaction.quantity;
          remaining -= transaction.quantity;
          self.transactions.remove(max_price_index);
      } else {
          gain_or_loss += (price - transaction.price) * remaining;
          transaction.quantity -= remaining;
          remaining = 0.0;
      }
  }

  gain_or_loss
}


fn calculate_gain_or_loss(
  transactions: Vec<Transaction>,
  method: String,
) -> Vec<Transaction> {
  let mut product = Product::new();
  let mut processed_transactions = Vec::new();

  for mut transaction in transactions {
    if transaction.action == "buy" {
      product.buy(transaction.quantity, transaction.price);
      processed_transactions.push(transaction);
    } else if transaction.action == "sell" {
      match method.as_str() {
        "fifo" => {
          transaction.profit =
            Some(product.sell_fifo(transaction.quantity, transaction.price));
          processed_transactions.push(transaction);
        }
        "lifo" => {
          transaction.profit =
            Some(product.sell_lifo(transaction.quantity, transaction.price));
          processed_transactions.push(transaction);
        }
        "hifo" => {
          transaction.profit =
            Some(product.hifo(transaction.quantity, transaction.price));
          processed_transactions.push(transaction);
        }
        _ => panic!("Invalid method! Please use \"fifo\" \"lifo\" or \"hifo\"."),
      }
    }
  }

  processed_transactions
}

fn main() {
  let transactions = vec![
    Transaction {
      action: "buy".to_string(),
      quantity: 10,
      price: 5.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 5,
      price: 8.0,
      profit: None,
    },
    Transaction {
      action: "buy".to_string(),
      quantity: 3,
      price: 20.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 2,
      price: 4.0,
      profit: None,
    },
    Transaction {
      action: "buy".to_string(),
      quantity: 2,
      price: 6.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 8,
      price: 14.0,
      profit: None,
    },
    // Add more transactions here
  ];
  let transactions2 = vec![
    Transaction {
      action: "buy".to_string(),
      quantity: 10,
      price: 5.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 5,
      price: 8.0,
      profit: None,
    },
    Transaction {
      action: "buy".to_string(),
      quantity: 3,
      price: 20.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 2,
      price: 4.0,
      profit: None,
    },
    Transaction {
      action: "buy".to_string(),
      quantity: 2,
      price: 6.0,
      profit: None,
    },
    Transaction {
      action: "sell".to_string(),
      quantity: 8,
      price: 14.0,
      profit: None,
    },
    // Add more transactions here
  ];

  let processed_transactions =
    calculate_gain_or_loss(transactions, "fifo".to_string());

  for transaction in processed_transactions {
    if let Some(profit) = transaction.profit {
      println!("fifo Gain or loss for this sell transaction: {}", profit);
    }
  }

  let processed_transactions =
    calculate_gain_or_loss(transactions2, "lifo".to_string());

  for transaction in processed_transactions {
    if let Some(profit) = transaction.profit {
      println!("lifo Gain or loss for this sell transaction: {}", profit);
    }
  }
  //TODO maybe here need to write about hifo
}
