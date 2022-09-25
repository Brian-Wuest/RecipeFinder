use async_trait::async_trait;
use futures::{future, StreamExt};
use tiberius::{ExecuteResult, Result, Row, ToSql};

use crate::DATA_POOL;

#[async_trait]
pub trait DataElement {
	async fn load_collection_with_params<'b>(query: &str, params: &'b [&'b dyn ToSql]) -> Vec<Self>
	where
		Self: Sized,
	{
		let connection = &mut DATA_POOL.get().await.unwrap();

		let initial_result = connection.query(query, params).await;

		let mut final_result: Vec<Self> = Vec::new();

		if let Ok(stream) = initial_result {
			let row_stream = stream.into_row_stream();

			let row_item = row_stream.for_each(|row| {
				if let Ok(data_row) = row {
					let populated_item = DataElement::populate_element_from_row(data_row);

					match populated_item {
						Some(data_element) => {
							final_result.push(data_element);
						}
						None => {}
					}
				}

				future::ready(())
			});

			row_item.await;
		}

		final_result
	}

	/// Loads a collection of data elements from the database and places them into a vector
	/// When there is an error in processing or if there are no results, an empty vector is returned.
	async fn load_collection(query: &str) -> Vec<Self>
	where
		Self: Sized,
	{
		let connection = &mut DATA_POOL.get().await.unwrap();

		let initial_result = connection.simple_query(query).await;

		let mut final_result: Vec<Self> = Vec::new();

		if let Ok(stream) = initial_result {
			let row_stream = stream.into_row_stream();

			let row_item = row_stream.for_each(|row| {
				if let Ok(data_row) = row {
					let populated_item = DataElement::populate_element_from_row(data_row);

					match populated_item {
						Some(data_element) => {
							final_result.push(data_element);
						}
						None => {}
					}
				}

				future::ready(())
			});

			row_item.await;
		}

		final_result
	}

	/// Loads the first record from the query and returns the data element or none if there was an error loading.
	async fn load_single(query: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match &mut DATA_POOL.get().await {
			Ok(connection) => {
				let initial_result = connection.simple_query(query).await;

				match initial_result {
					Ok(stream) => {
						let data_row = stream.into_row().await.unwrap();

						match data_row {
							Some(row) => DataElement::populate_element_from_row(row),
							None => Option::None,
						}
					}
					Err(_) => Option::None,
				}
			}
			Err(error) => {
				dbg!(error);
				panic!("Error during execution, see debug log for more information");
			}
		}
	}

	/// Loads the first record from the query and returns the data element or none if there was an error loading.
	async fn load_single_with_params<'b>(query: &str, params: &'b [&'b dyn ToSql]) -> Option<Self>
	where
		Self: Sized,
	{
		match &mut DATA_POOL.get().await {
			Ok(connection) => {
				let initial_result = connection.query(query, params).await;

				match initial_result {
					Ok(stream) => {
						let data_row = stream.into_row().await.unwrap();

						match data_row {
							Some(row) => DataElement::populate_element_from_row(row),
							None => Option::None,
						}
					}
					Err(_) => Option::None,
				}
			}
			Err(error) => {
				dbg!(error);
				panic!("Error during execution, see debug log for more information");
			}
		}
	}

	async fn delete(query: &str) -> Result<ExecuteResult> {
		let connection = &mut DATA_POOL.get().await.unwrap();
		connection.execute(query, &[]).await
	}

	async fn delete_with_params<'b>(query: &str, params: &'b [&'b dyn ToSql]) -> Result<ExecuteResult> {
		let connection = &mut DATA_POOL.get().await.unwrap();
		connection.execute(query, params).await
	}

	async fn insert(query: &str) -> Result<ExecuteResult> {
		let connection = &mut DATA_POOL.get().await.unwrap();
		connection.execute(query, &[]).await
	}

	async fn insert_with_params<'b>(query: &str, params: &'b [&'b dyn ToSql]) -> Result<ExecuteResult> {
		let connection = &mut DATA_POOL.get().await.unwrap();
		connection.execute(query, params).await
	}

	// Populates the data element from the specified Row
	fn populate_element_from_row(row: Row) -> Option<Self>
	where
		Self: Sized;
}
