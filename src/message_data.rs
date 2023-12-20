use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

pub(crate) trait MessageData: Any {
    fn title(&self) -> &str;
    fn logo_url(&self) -> &str;
    fn progress(&self) -> &str;
    fn goal(&self) -> &str;
    fn rate(&self) -> &str;
}

impl MessageData for Ikap {
    fn title(&self) -> &str {
        &self.title
    }

    fn logo_url(&self) -> &str {
        &self.img
    }

    fn progress(&self) -> &str {
        &self.progress
    }

    fn goal(&self) -> &str {
        &self.target.goal_value
    }

    fn rate(&self) -> &str {
        &self.target.rate_value
    }
}

impl MessageData for GoCrowd {
    fn title(&self) -> &str {
        &self.offering_name
    }

    fn logo_url(&self) -> &str {
        &self.logo
    }

    fn progress(&self) -> &str {
        &self.current_reserved_amount_str
    }

    fn goal(&self) -> &str {
        &self.max_target_str
    }

    fn rate(&self) -> &str {
        &self.interest_rate_str
    }
}

impl GoCrowd {
    pub fn update_current_reserved_amount_str(&mut self) {
        self.current_reserved_amount_str = self.current_reserved_amount.to_string();
    }
    pub fn update_max_target_str(&mut self) {
        self.max_target_str = self.max_target.to_string();
    }
    pub fn update_interest_rate_str(&mut self) {
        self.interest_rate_str = self.interest_rate.to_string();
    }
}
pub type VecGoCrowd = Vec<GoCrowd>;
pub type VecIkap = Vec<Ikap>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ikap {
    pub title: String,
    pub img: String,
    pub progress: String,
    pub target: Target,
    pub renew: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub(crate) goal_value: String,
    pub(crate) rate_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoCrowd {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub state: String,
    #[serde(rename = "offering_name")]
    pub offering_name: String,
    #[serde(rename = "min_invest_amount")]
    pub min_invest_amount: u64,
    #[serde(rename = "min_target")]
    pub min_target: u64,
    #[serde(rename = "max_target")]
    pub max_target: u64,
    #[serde(default)]
    pub max_target_str: String,
    #[serde(rename = "total_investors")]
    pub total_investors: i64,
    #[serde(rename = "investment_contract_type")]
    pub investment_contract_type: String,
    #[serde(rename = "current_reserved_amount")]
    pub current_reserved_amount: u64,
    #[serde(default)]
    pub current_reserved_amount_str: String,
    #[serde(rename = "funded_amount")]
    pub funded_amount: i64,
    #[serde(rename = "reserved_investors")]
    pub reserved_investors: i64,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "raising_duration")]
    pub raising_duration: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "advertising_description")]
    pub advertising_description: String,
    pub logo: String,
    #[serde(rename = "payment_start_on")]
    pub payment_start_on: Value,
    #[serde(rename = "interest_rate")]
    pub interest_rate: i64,
    #[serde(default)]
    pub interest_rate_str: String,
    pub guarantor: String,
    #[serde(rename = "repayment_frequency")]
    pub repayment_frequency: String,
    #[serde(rename = "months_until_maturity")]
    pub months_until_maturity: i64,
    pub compound: String,
    #[serde(rename = "grace_period_months")]
    pub grace_period_months: Option<i64>,
    #[serde(rename = "payment_schedule_enabled")]
    pub payment_schedule_enabled: bool,
    #[serde(rename = "state_i18n")]
    pub state_i18n: String,
}

pub fn process_item(item: &GoCrowd) -> Vec<(&str, String)> {
    vec![
        ("id", item.id.to_string()),
        ("type", item.type_field.to_string()),
        ("state", item.state.to_string()),
        ("offering_name", item.offering_name.to_string()),
        ("min_invest_amount", item.min_invest_amount.to_string()),
        ("min_target", item.min_target.to_string()),
        ("max_target", item.max_target.to_string()),
        ("total_investors", item.total_investors.to_string()),
        (
            "investment_contract_type",
            item.investment_contract_type.to_string(),
        ),
        (
            "current_reserved_amount",
            item.current_reserved_amount.to_string(),
        ),
        ("funded_amount", item.funded_amount.to_string()),
        ("reserved_investors", item.reserved_investors.to_string()),
        ("start_date", item.start_date.to_string()),
        ("raising_duration", item.raising_duration.to_string()),
        ("end_date", item.end_date.to_string()),
        (
            "advertising_description",
            item.advertising_description.to_string(),
        ),
        ("logo", item.logo.to_string()),
        ("payment_start_on", item.payment_start_on.to_string()),
        ("interest_rate", item.interest_rate.to_string()),
        ("guarantor", item.guarantor.to_string()),
        ("repayment_frequency", item.repayment_frequency.to_string()),
        (
            "months_until_maturity",
            item.months_until_maturity.to_string(),
        ),
        ("compound", item.compound.to_string()),
        (
            "grace_period_months",
            item.grace_period_months.unwrap_or_default().to_string(),
        ),
        (
            "payment_schedule_enabled",
            item.payment_schedule_enabled.to_string(),
        ),
        ("state_i18n", item.state_i18n.to_string()),
    ]
}
