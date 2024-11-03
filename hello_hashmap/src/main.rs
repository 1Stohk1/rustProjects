mod functions;

use crate::functions::data_science;
use crate::functions::dep;

fn main() {
    dep::company_dep();
    data_science::calculate();
}