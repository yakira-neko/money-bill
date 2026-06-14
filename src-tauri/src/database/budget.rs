use rusqlite::{params, Connection, Result};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

/// 预算记录
///
/// 一个预算针对某个支出账户（或账户前缀）在某个周期内设定上限金额。
/// - `account` 为空字符串或 "expenses" 表示「总预算」（统计所有支出）。
/// - `account` 形如 "expenses::food" 表示只统计该分类及其子分类的支出。
#[derive(Debug, Clone)]
pub struct Budget {
    pub id: String,
    pub account: String,
    pub amount: Decimal,
    pub period: String,
    pub currency: String,
}

/// 新增一条预算，返回生成的预算 id
pub fn add_budget(
    conn: &Connection,
    account: &str,
    amount: Decimal,
    period: &str,
    currency: &str,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO BUDGET (id,account,amount,period,currency) VALUES (?1,?2,?3,?4,?5)",
        params![id, account, amount.to_f32(), period, currency],
    )?;
    Ok(id)
}

/// 获取全部预算
pub fn get_budgets(conn: &Connection) -> Result<Vec<Budget>> {
    let mut stmt = conn.prepare("SELECT id,account,amount,period,currency FROM BUDGET")?;
    let iter = stmt.query_map([], |row| {
        Ok(Budget {
            id: row.get(0)?,
            account: row.get(1)?,
            amount: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap_or(Decimal::ZERO),
            period: row.get(3)?,
            currency: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for i in iter {
        result.push(i?);
    }
    Ok(result)
}

/// 按 id 读取单条预算
pub fn get_budget_by_id(conn: &Connection, id: &str) -> Result<Option<Budget>> {
    let mut stmt =
        conn.prepare("SELECT id,account,amount,period,currency FROM BUDGET WHERE id=?1")?;
    let mut iter = stmt.query_map(params![id], |row| {
        Ok(Budget {
            id: row.get(0)?,
            account: row.get(1)?,
            amount: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap_or(Decimal::ZERO),
            period: row.get(3)?,
            currency: row.get(4)?,
        })
    })?;
    if let Some(row) = iter.next() {
        Ok(Some(row?))
    } else {
        Ok(None)
    }
}

/// 更新预算
pub fn update_budget(
    conn: &Connection,
    id: &str,
    account: &str,
    amount: Decimal,
    period: &str,
    currency: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE BUDGET SET account=?2,amount=?3,period=?4,currency=?5 WHERE id=?1",
        params![id, account, amount.to_f32(), period, currency],
    )?;
    Ok(())
}

/// 删除预算
pub fn del_budget(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM BUDGET WHERE id=?1", params![id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::init;
    use rust_decimal_macros::dec;

    #[test]
    fn test_add_and_get_budget() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        add_budget(&conn, "expenses::food", dec!(1000.0), "monthly", "CNY")?;
        let budgets = get_budgets(&conn)?;
        assert_eq!(1, budgets.len());
        assert_eq!("expenses::food", budgets[0].account);
        assert_eq!(dec!(1000.0), budgets[0].amount);
        Ok(())
    }

    #[test]
    fn test_update_budget() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        let id = add_budget(&conn, "expenses::food", dec!(1000.0), "monthly", "CNY")?;
        update_budget(&conn, &id, "expenses::transport", dec!(500.0), "monthly", "CNY")?;
        let budget = get_budget_by_id(&conn, &id)?.unwrap();
        assert_eq!("expenses::transport", budget.account);
        assert_eq!(dec!(500.0), budget.amount);
        Ok(())
    }

    #[test]
    fn test_del_budget() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init::init(&conn)?;
        let id = add_budget(&conn, "expenses::food", dec!(1000.0), "monthly", "CNY")?;
        assert_eq!(1, get_budgets(&conn)?.len());
        del_budget(&conn, &id)?;
        assert_eq!(0, get_budgets(&conn)?.len());
        Ok(())
    }
}
