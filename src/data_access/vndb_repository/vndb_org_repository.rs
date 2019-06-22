use crate::data_access::vndb_repository::VNDBRepository;
use crate::models::vndb_result::VNDBResult;
use html5ever;
use html5ever::rcdom::{Handle, NodeData};
use html5ever::tendril::TendrilSink;
use reqwest;
use reqwest::StatusCode;

pub struct VNDBOrgRepository;

#[derive(Debug)]
struct VNDBOrgRepositoryError {
    description: String,
}

fn find_top_table_result(node: &Handle) -> Option<String> {
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if &name.local == "table" {
                for attribute in attrs.borrow().iter() {
                    if (&attribute.name.local == "class") && (attribute.value == html5ever::tendril::Tendril::from("stripe")) {
                        let tbody_index = 1;
                        let node_children = node.children.borrow();
                        let tbody = node_children.get(tbody_index)?;
                        let tbody_children = tbody.children.borrow();
                        let first_row = &tbody_children.first()?;
                        let first_row_children = first_row.children.borrow();
                        let first_column = first_row_children.first()?;
                        let first_column_children = first_column.children.borrow();
                        let link_to_vn_element = first_column_children.first()?;

                        match link_to_vn_element.data {
                            NodeData::Element {
                                ref attrs,
                                ..
                            } => {
                                for link_attribute in attrs.borrow().iter() {
                                    if &link_attribute.name.local == "href" {
                                        return Some(format!("{}", attribute.value));
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }

            let mut result = None;
            for child in node.children.borrow().iter() {
                if let Some(value) = find_top_table_result(child) {
                    result = Some(value)
                };
            }
            result
        }
        _ => None,
    }
}

fn format_search_parameters(parameters: Vec<&str>) -> String {
    parameters.join("+")
}

fn get_recomended_link(body: &str) -> Result<String, Box<std::error::Error>> {
    let dom = html5ever::parse_document(
        html5ever::rcdom::RcDom::default(),
        html5ever::ParseOpts {
            tree_builder: html5ever::tree_builder::TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .from_utf8()
    .read_from(&mut body.as_bytes())?;

    let result = find_top_table_result(&dom.document).ok_or_else(|| {
        VNDBOrgRepositoryError::new(String::from("unable to find table on vndb result page"))
    })?;

    Ok(result)
}

impl VNDBOrgRepository {
    pub fn default() -> VNDBOrgRepository {
        VNDBOrgRepository
    }
}

impl VNDBRepository for VNDBOrgRepository {
    fn get_visual_novel(&self, query: Vec<&str>) -> Result<VNDBResult, Box<std::error::Error>> {
        let mut result;

        let formated_params = format_search_parameters(query);
        let request_uri = format!("https://vndb.org/v/all?sq={}", formated_params);

        let mut response: reqwest::Response = reqwest::Client::builder()
            .redirect(reqwest::RedirectPolicy::none())
            .build()?
            .get(&request_uri)
            .send()?;
        
        match response.status() {
            StatusCode::TEMPORARY_REDIRECT => {
                let location_header = response
                    .headers()
                    .get(reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        VNDBOrgRepositoryError::new(String::from(
                            "VNDB API returned redirect but there was no location header",
                        ))
                    })?
                    .to_str()?
                    .to_owned();
                result = VNDBResult::Single(format!("https://vndb.org{}", location_header));
            }
            StatusCode::OK => {
                let recomended_link = get_recomended_link(&response.text()?)?;

                result = VNDBResult::MostLikelyAndMore(format!("https://vndb.org{}", recomended_link), request_uri);
            }
            _ => {
                result = VNDBResult::None;
            }
        }

        Ok(result)
    }
}

impl VNDBOrgRepositoryError {
    fn new(description: String) -> VNDBOrgRepositoryError {
        VNDBOrgRepositoryError { description }
    }
}

impl std::error::Error for VNDBOrgRepositoryError {}

impl std::fmt::Display for VNDBOrgRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
