use advent_of_code::solve;
use std::convert::TryInto;
use tonic::{transport::Server, Code, Request, Response, Status};

use advent::solver_server::{Solver, SolverServer};
use advent::{ProblemInput, ProblemOutput};

mod advent {
    #![allow(warnings)]
    #![allow(clippy)]
    tonic::include_proto!("advent");
}

#[derive(Default)]
pub struct SolverImpl {}

#[tonic::async_trait]
impl Solver for SolverImpl {
    async fn solve(
        &self,
        request: Request<ProblemInput>,
    ) -> Result<Response<ProblemOutput>, Status> {
        let input: ProblemInput = request.into_inner();

        let year: u16 = match input.year.try_into() {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::new(Code::OutOfRange, "year too big"));
            }
        };

        let day: u8 = match input.day.try_into() {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::new(Code::OutOfRange, "day too big"));
            }
        };

        let part: u8 = match input.part.try_into() {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::new(Code::OutOfRange, "part too big"));
            }
        };

        let result = solve(year, day, part, &input.text);

        match result {
            Err(error_string) => Err(Status::new(Code::InvalidArgument, error_string)),
            Ok(text) => Ok(Response::new(ProblemOutput { text })),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #![allow(clippy::unwrap_used)]

    let addr = "0.0.0.0:50051".parse()?;
    let solver = SolverImpl::default();

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(SolverServer::new(solver))
        .serve(addr)
        .await?;

    Ok(())
}
