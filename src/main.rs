struct FilterCondition<'a, T: PartialEq> {
    field: &'a T,
}

impl<'a, T: PartialEq> FilterCondition<'a, T> {
    fn new(field: &'a T) -> FilterCondition<'a, T> {
        FilterCondition { field }
    }

    fn is_match(&self, other: &T) -> bool {
        self.field == other
    }
}

fn custom_filter<'a, T: PartialEq>(collection: &'a [T], filter_condition: &FilterCondition<T>) -> Vec<&'a T> {
    collection.iter().filter(|&item| filter_condition.is_match(item)).collect()
}

fn main() {
    let collection: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_value = 5;
    let filter_condition = FilterCondition::new(&filter_value);
    let filtered_result = custom_filter(&collection, &filter_condition);
    println!("Filtered result: {:?}", filtered_result);
}

