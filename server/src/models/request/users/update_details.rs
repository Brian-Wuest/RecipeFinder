/*!
Filename: c:\Users\Brian\Documents\GitHub\RecipeFinder\server\src\models\request\users\update_details.rs
Path: c:\Users\Brian\Documents\GitHub\RecipeFinder\server\src\models\request\users
Created Date: Wednesday, August 17th 2022, 6:16:40 am
Author: Brian Wuest

Copyright (c) 2022 Your Company
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDetailsRequest {
	pub name: Option<String>,
	pub email: Option<String>,
}
