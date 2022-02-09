pub enum Plan {
    DDL,
    Update,
    Insert,
    Delete,
    LogicalPlan(LogicalPlan),
}

pub enum LogicalPlan {
    Filter,
    Projection,
    Join,
    Aggregation,
    Sort,
    TopN,
}
