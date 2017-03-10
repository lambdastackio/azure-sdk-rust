# Microsoft Azure SDK for Rust

## Introduction
Microsoft Azure expose its technologies via REST API. These APIs are easily consumable from any language (good) but are weakly typed.

## Disclaimer
This project was heavily influenced by azure_sdk_for_rust crate but the code has not been updated in over a year.

*Do not use at this time or until the next point release*

## Example
You can find examples in the test section (not yet existent as far as Azure is concerned) and in the main.rs file. Here is a sample however:

### main.rs
```rust
extern crate azure_sdk_rust;
extern crate chrono;
#[macro_use]
extern crate mime;

use azure_sdk_for_rust::azure::core::lease::{LeaseState, LeaseStatus};
use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT};
use azure_sdk_for_rust::azure::storage::container::{Container, PublicAccess, LIST_CONTAINER_OPTIONS_DEFAULT};

use chrono::UTC;

use mime::Mime;

fn main() {
  let azure_storage_account = &"azure_storage_account";
  let azure_storage_key= &"azure_storage_key";

  // create the client struct. The third argument, if false, forces to use
  // http instead of https. It's useful if you have trouble compiling
  // hyper with openSSL activated.
  let client = Client::new(azure_storage_account, azure_storage_key, false);


  // This call will list your containers.
  let containers = Container::list(&client, &LIST_CONTAINER_OPTIONS_DEFAULT).unwrap();
  println!("{:?}", containers);

  let container_name = "rust";
  // This call will create a new Azure Container called "wow"
  // with public blob access (see https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx)
  // if it doesn't exist already.

  let cont = containers.iter().find(|x| x.name == container_name);
  if let None = cont {
  	Container::create(&client, container_name, PublicAccess::Blob).unwrap();
  }

  // this code will upload a file to the container just created.
  {
	use std::fs::metadata;
	use std::fs::File;

	let file_name: &'static str = "C:\\temp\\from_rust.txt";
	let container_name: &'static str = "wow";

	let metadata = metadata(file_name).unwrap();
	let mut file = File::open(file_name).unwrap();

	let new_blob = Blob {
		name: "from_rust.txt".to_owned(),
        container_name: container_name.to_owned(),
		snapshot_time: None,
		last_modified: UTC::now(),
		etag: "".to_owned(),
		content_length: metadata.len(),
		content_type: "application/octet-stream".parse::<Mime>().unwrap(),
		content_encoding: None,
		content_language: None,
		content_md5: None,
		cache_control: None,
		x_ms_blob_sequence_number: None,
		blob_type: BlobType::BlockBlob,
		lease_status: LeaseStatus::Unlocked,
		lease_state: LeaseState::Available,
		lease_duration: None,
		copy_id: None,
		copy_status: None,
		copy_source: None,
		copy_progress: None,
		copy_completion: None,
		copy_status_description: None,
	};

	new_blob.put(&client,
        &PUT_OPTIONS_DEFAULT,
		 Some((&mut file, metadata.len())))
		.unwrap();
  }


  // This code will look for the "todelete" container and
  // remove from Azure.
  let mut to_delete = containers.iter_mut().find(|x| x.name == "todelete").unwrap();
  to_delete.delete(&client).unwrap();
  println!("{:?} deleted!", to_delete);
}
```


#### Storage Container
Lifted from azure_sdk_for_rust project:

|Method | URL |
| ----  | --- |
|Create container|[https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179468.aspx)|
|List containers|[https://msdn.microsoft.com/en-us/library/azure/dd179352.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179352.aspx)|
|Delete container|[https://msdn.microsoft.com/en-us/library/azure/dd179408.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179408.aspx)|

#### Storage blobs

|Method | URL |
| ----  | --- |
|List blobs|[https://msdn.microsoft.com/en-us/library/azure/dd135734.aspx](https://msdn.microsoft.com/en-us/library/azure/dd135734.aspx)|
|Get blob|[https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179440.aspx)|
|Put blob|[https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx)|
|Put blob page|[https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx)|
|Clear blob page|[https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx](https://msdn.microsoft.com/en-us/library/azure/dd179451.aspx)|
|Put block|[https://msdn.microsoft.com/en-us/library/azure/dd135726.aspx](https://msdn.microsoft.com/en-us/library/azure/dd135726.aspx)|
|Lease blob|[https://msdn.microsoft.com/library/azure/ee691972.aspx](https://msdn.microsoft.com/library/azure/ee691972.aspx)|

#### Event Hubs

|Method | URL |
| ----  | --- |
|Send Event|[https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx](https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx)|


## License
This project is published under [Apache license, version 2.0](LICENSE).
