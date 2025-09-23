use crate::event_error::EventResult;

#[derive(Clone, Debug)]
pub struct EmailStruct {
    pub from: String,
    pub to: String,
    pub content: String,
}

pub struct EmailService {
    pub sender: String,
}

impl EmailService {
    pub async fn send_email(&self, _e: EmailStruct) -> EventResult<()> {
        Ok(())
    }
}

pub(crate) fn generate_cancellation_email_simple(
    organizer_name: &str,
    lessor_name: &str,
    venue_name: &str,
) -> String {
    format!(
        r#"尊敬的 {}，

您好！

我们特此通知您，用户{}已取消了对场地{}的租赁请求。

您无需进行任何操作。该场地现在可供其他用户预订。

此致，

VenueFlow团队"#,
        lessor_name, organizer_name, venue_name
    )
}

pub(crate) fn generate_cancellation_email_simple_for_organizer(
    organizer_name: &str,
    lessor_name: &str,
    venue_name: &str,
) -> String {
    format!(
        r#"尊敬的 {}，

您好！

我们特此通知您,您已经已取消了对用户{}的场地{}的租赁请求。

此为通知邮件，您无需进行任何操作。

此致，

VenueFlow团队"#,
        organizer_name, lessor_name, venue_name
    )
}

pub(crate) fn generate_rental_email_simple(
    organizer_name: &str,
    venue_name: &str,
    status: &str,
) -> String {
    format!(
        r#"尊敬的 {}，

您好！

我们特此通知您,您对场地{}的租赁请求为{}

此为通知邮件，您无需进行任何操作。

此致，

VenueFlow团队"#,
        organizer_name, status, venue_name
    )
}
