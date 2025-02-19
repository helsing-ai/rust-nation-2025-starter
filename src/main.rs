use drone::{DroneClientMsg, DroneServerMsg};
use eyre::Result;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

pub mod drone {
    tonic::include_proto!("drone");
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = format!("Bearer {}", env!("USER"));
    println!("{:?}", token);

    let channel = Channel::from_static("http://172.236.8.116:10301")
        .connect()
        .await?;
    let token: MetadataValue<_> = token.parse()?;
    let mut client = drone::drone_controller_client::DroneControllerClient::with_interceptor(
        channel,
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    let (req_tx, req_rx) = mpsc::channel(128);
    let request_stream = ReceiverStream::new(req_rx);
    let mut response_stream: tonic::Streaming<DroneServerMsg> =
        client.drone_connection(request_stream).await?.into_inner();

    loop {
        req_tx
            .send(DroneClientMsg {
                throttle: 65,
                roll: 0,
                pitch: 0,
            })
            .await?;
    }

    // you can now receive messages as follows:
    // let message = response_stream.message().await;
    // and send messages with
    // req_tx.send(...).await?;

    // check out the proto file - drone.proto - for hints on what types you
    // should be using, and the limits of throttle, pitch, and roll

    // go to http://172.236.8.116:8080 in a browser to see live results!
    Ok(())
}
