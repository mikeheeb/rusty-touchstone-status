use serde::{Deserialize, Serialize};

fn send_request() -> String {
    let client_builder = reqwest::blocking::ClientBuilder::new();
    let client = client_builder.build().unwrap();

    client
        .get("http://192.168.100.1/cgi-bin/status_cgi")
        .send()
        .unwrap()
        .text()
        .unwrap()
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct RFParameters {
    downstream_parameters: Vec<DownstreamParameter>,
    upstream_parameters: Vec<UpstreamParameter>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct UpstreamParameter {
    id: String,
    channel_id: u8,
    frequency: String,
    power: String,
    channel_type: String,
    symbol_rate: String,
    modulation: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DownstreamParameter {
    id: u8,
    channel_id: u8,
    freq: String,
    power: String,
    snr: String,
    modulation: String,
    octets: usize,
    correcteds: u8,
    uncorrectables: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct StatusParameter {
    uptime: String,
    computers_detected: String,
    cm_status: String,
    current_datetime: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct InterfaceInformation {
    name: String,
    provisioned: String,
    state: String,
    speed: String,
    mac_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ArrisStatus {
    rf_parameters: RFParameters,
    status: StatusParameter,
    interfaces: Vec<InterfaceInformation>,
}

fn parse_tables_rf_parameters_upstream_table(
    tables_rf_parameters_upstream: scraper::ElementRef,
) -> Vec<UpstreamParameter> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut tables_rf_parameters_upstream_iter = tables_rf_parameters_upstream.select(&tr_selector);

    // Skip first row.
    tables_rf_parameters_upstream_iter.next();

    let mut rf_parameters_tabledata = tables_rf_parameters_upstream_iter
        .next()
        .unwrap()
        .select(&td_selector);

    let upstream_parameter = UpstreamParameter {
        id: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat()
            .split(' ')
            .last()
            .unwrap()
            .to_string(),

        channel_id: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat()
            .parse::<u8>()
            .unwrap_or(0),

        frequency: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat(),

        power: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat(),

        channel_type: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat(),

        symbol_rate: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat(),

        modulation: rf_parameters_tabledata
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat(),
    };

    return vec![upstream_parameter];
}

fn parse_tables_rf_parameters_downstream_table(
    tables_rf_parameters_downstream: scraper::ElementRef,
) -> Vec<DownstreamParameter> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut tables_rf_parameters_downstream_iter =
        tables_rf_parameters_downstream.select(&tr_selector);

    // Skip first row.
    tables_rf_parameters_downstream_iter.next();

    let mut results: Vec<DownstreamParameter> = Vec::new();

    for datarow in tables_rf_parameters_downstream_iter {
        let mut rf_parameters_tds = datarow.select(&td_selector);

        let downstream_parameter = DownstreamParameter {
            id: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .split(' ')
                .last()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),

            channel_id: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .parse::<u8>()
                .unwrap_or(0),

            freq: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat(),

            power: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat(),

            snr: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat(),

            modulation: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat(),

            octets: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .parse::<usize>()
                .unwrap_or(0),

            correcteds: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .parse::<u8>()
                .unwrap_or(0),

            uncorrectables: rf_parameters_tds
                .next()
                .unwrap()
                .text()
                .collect::<Vec<&str>>()
                .concat()
                .parse::<u8>()
                .unwrap_or(0),
        };

        results.push(downstream_parameter);
    }

    results
}

fn parse_tables_interface_parameters_table(
    tables_interface_parameters: scraper::ElementRef,
) -> Vec<InterfaceInformation> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut tables_interface_parameters_iter = tables_interface_parameters.select(&tr_selector);

    // Skip first row.
    tables_interface_parameters_iter.next();

    tables_interface_parameters_iter.fold(Vec::new(), |mut results, datarow| {
        let mut interface_parameter = InterfaceInformation::default();

        let mut rf_parameters_tds = datarow.select(&td_selector);

        interface_parameter.name = rf_parameters_tds
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat();

        interface_parameter.provisioned = rf_parameters_tds
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat();

        interface_parameter.state = rf_parameters_tds
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat();

        interface_parameter.speed = rf_parameters_tds
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat();

        interface_parameter.mac_address = rf_parameters_tds
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .concat();

        results.push(interface_parameter);

        results
        
    })
}

fn parse_tables_status_table(tables_status: scraper::ElementRef) -> StatusParameter {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut tables_status_iter = tables_status.select(&tr_selector);

    StatusParameter {
        uptime: tables_status_iter
            .next()
            .unwrap()
            .select(&td_selector)
            .last()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .concat(),

        computers_detected: tables_status_iter
            .next()
            .unwrap()
            .select(&td_selector)
            .last()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .concat()
            .trim()
            .to_owned(),

        cm_status: tables_status_iter
            .next()
            .unwrap()
            .select(&td_selector)
            .last()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .concat(),

        current_datetime: tables_status_iter
            .next()
            .unwrap()
            .select(&td_selector)
            .last()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .concat(),
    }
}

fn parse_request(html: &str) -> ArrisStatus {
    let document = scraper::Html::parse_document(html);

    let table_selector = scraper::Selector::parse("table").unwrap();

    let mut tables = document.select(&table_selector);

    let _tables_rf_parameters = tables.next().unwrap();

    let tables_rf_parameters_downstream = tables.next().unwrap();
    let downstream_parameters =
        parse_tables_rf_parameters_downstream_table(tables_rf_parameters_downstream);

    let _tables_rf_parameters_fec_counters = tables.next().unwrap();

    let tables_rf_parameters_upstream = tables.next().unwrap();
    let upstream_parameters =
        parse_tables_rf_parameters_upstream_table(tables_rf_parameters_upstream);

    let _tables_status = tables.next().unwrap();

    let tables_status = tables.next().unwrap();
    let tables_status_values = parse_tables_status_table(tables_status);

    let _tables_interface_parameters = tables.next().unwrap();

    let tables_interface_parameters_values = tables.next().unwrap();
    let interface_parameters =
        parse_tables_interface_parameters_table(tables_interface_parameters_values);

    let rf_parameters = RFParameters {
        downstream_parameters,
        upstream_parameters,
    };

    ArrisStatus {
        rf_parameters,
        status: tables_status_values,
        interfaces: interface_parameters,
    }
}

fn main() {
    let blah = send_request();
    let html = blah.as_str();

    let parsed = parse_request(html);
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}
