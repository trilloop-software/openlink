// valididate connection
macro_rules! conn_test(
    ($c:expr) => {
        match $c {
            Some(c) => c,
            None => return Err(s!["Please connect to the Pod Computer first."])
        }
    }
);

// transform into a string
macro_rules! s( ($e:expr) => ( ($e).to_string() ) );
