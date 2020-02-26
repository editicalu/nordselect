use nordselect::filters::Filter;
use nordselect::Servers;

pub fn apply_filters(filters_to_apply: Vec<Box<dyn Filter>>, data: &mut Servers) {
    for filter in filters_to_apply.iter() {
        data.filter(filter.as_ref())
    }
}
