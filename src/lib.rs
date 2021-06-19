#[derive(Debug, Clone)]
pub struct DataPoint<Time, Data>
where
    Time: Clone,
    Data: Clone,
{
    pub time: Time,
    pub data: Data,
}

impl<Time, Data> DataPoint<Time, Data>
where
    Time: Clone,
    Data: Clone,
{
    pub fn new(time: Time, data: Data) -> DataPoint<Time, Data> {
        DataPoint { time, data }
    }
}

#[derive(Debug, Clone)]
pub struct TimeSeries<Time, Data>
where
    Time: Clone,
    Data: Clone,
{
    inner: Vec<DataPoint<Time, Data>>,
}

impl<Time, Data> TimeSeries<Time, Data>
where
    Time: Clone,
    Data: Clone,
{
    pub fn new() -> TimeSeries<Time, Data> {
        TimeSeries { inner: Vec::new() }
    }

    pub fn add_point(&mut self, data_point: DataPoint<Time, Data>) {
        self.inner.push(data_point);
    }

    pub fn to_vec(&self) -> Vec<DataPoint<Time, Data>> {
        self.inner.clone()
    }
}
