use rusqlite::{self, Connection, Result};

pub fn init(conn: &Connection) -> Result<()> {
    init_account(conn)?;
    init_transaction(conn)?;
    init_details(conn)?;
    init_budget(conn)?;
    Ok(())
}

fn init_account(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE ACCOUNT(
        name VARCHAR(255) NOT NULL PRIMARY KEY ,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL,
        icon VARCHAR(255),
        extra VARCHAR(255) 
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_details(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE DETAIL(
        id TEXT PRIMARY KEY,
        trans_id TEXT NOT NULL,
        account TEXT NOT NULL,
        currency VARCHAR(255) NOT NULL,
        balance float NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

fn init_transaction(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE TRANS(
        id TEXT PRIMARY KEY,
        time TEXT NOT NULL,
        extra VARCHAR(255)
    )
    ",
        (),
    )?;
    Ok(())
}

/// 初始化预算表。
///
/// 使用 `CREATE TABLE IF NOT EXISTS`，因此对已存在 db.db3 的老用户也可安全地补建该表。
///
/// 字段说明：
/// - `id`: 预算唯一标识 (UUID)
/// - `account`: 关联的支出账户/前缀，空字符串表示总预算
/// - `amount`: 预算上限金额
/// - `period`: 预算周期，目前支持 "monthly"
/// - `currency`: 币种
pub fn init_budget(conn: &Connection) -> Result<()> {
    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS BUDGET(
        id TEXT PRIMARY KEY,
        account VARCHAR(255) NOT NULL,
        amount FLOAT NOT NULL,
        period VARCHAR(255) NOT NULL,
        currency VARCHAR(255) NOT NULL
    )
    ",
        (),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};
    use rust_decimal::Decimal;

    use crate::database::{
        account::{Account},
        details::Details,
        transaction::Transaction,
    };

    use super::*;
    #[test]
    fn test_account() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_account(&conn)?;
        conn.execute(
            "INSERT INTO ACCOUNT (name,currency,balance) VALUES (?1,?2,?3)",
            ( "a", "b", 0.1),
        )?;
        let mut stmt = conn.prepare("SELECT name,currency,balance FROM ACCOUNT")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Account {
                name: row.get(0)?,
                currency: row.get(1)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(2)?).unwrap(),
                icon: row.get(3)?,
                extra:row.get(4)?,
            })
        })?;
        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }

    #[test]
    fn test_details() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_details(&conn)?;
        conn.execute(
            "INSERT INTO DETAIL (id,trans_id,account,currency,balance) VALUES (?1,?2,?3,?4,?5)",
            (0, 1, 2, "CNY", 0.1),
        )?;
        let mut stmt =
            conn.prepare("SELECT id,trans_id,account,currency,balance FROM DETAIL")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Details {
                id: row.get(0)?,
                trans_id: row.get(1)?,
                account: row.get(2)?,
                currency: row.get(3)?,
                balance: Decimal::from_f32_retain(row.get::<usize, f32>(4)?).unwrap(),
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }

    #[test]
    fn test_trans() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_transaction(&conn)?;
        conn.execute(
            "INSERT INTO TRANS (id,time,extra) VALUES (?1,?2,?3)",
            (0, "2023-08-21", "CNY"),
        )?;
        let mut stmt = conn.prepare("SELECT id,extra FROM TRANS")?;
        let mut iter = stmt.query_map([], |row| {
            Ok(Transaction {
                id: row.get(0)?,
                date: row.get(1)?,
                extra: row.get(2)?,
            })
        })?;

        if iter.next().is_some() {
            return Ok(());
        }
        panic!()
    }
}
