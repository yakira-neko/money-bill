use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WeeklyIncomeExpenses {
    pub income: Vec<f32>,
    pub expenses: Vec<f32>,
}
#[derive(Serialize, Deserialize)]
pub struct AccountIconName {
    pub name:String,
    pub icon:String,
    pub currency:String,
    pub balance: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AccountAmount {
    pub account: String,
    pub amount: f32,
}

/// 预算项及其执行进度，返回给前端展示。
#[derive(Serialize, Deserialize)]
pub struct BudgetProgress {
    /// 预算唯一 id
    pub id: String,
    /// 关联的支出账户/前缀，空字符串表示总预算
    pub account: String,
    /// 预算上限金额
    pub amount: f32,
    /// 本周期已花费金额
    pub spent: f32,
    /// 预算周期，如 "monthly"
    pub period: String,
    /// 币种
    pub currency: String,
}
