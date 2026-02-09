use crate::stocks::data_company::StockData;

#[derive(Debug, PartialEq)]
pub enum CandlestickType {
    LongRedCandle,
    LongGreenCandle,
    Doji,
    LongLowerShadow,
    LongUpperShadow,
    // ShootingStar,
    // SpinningTop,
    Unknown,
}

pub fn anal_candlestick(stock_data: &StockData) -> CandlestickType {
    let open = stock_data.open;
    let close = stock_data.close;
    let high = stock_data.high;
    let low = stock_data.low;

    let body_length = (close - open).abs();
    let upper_shadow = high - open.max(close);
    let lower_shadow = open.min(close) - low;

    if body_length > 0.05 * close {
        if close > open {
            CandlestickType::LongRedCandle
        } else {
            CandlestickType::LongGreenCandle
        }
    } else if (open - close).abs() < 0.01 * ((high - low).max(1.0))
        && open != high
        && open != low
        && close != high
        && close != low
    {
        CandlestickType::Doji
    } else if lower_shadow > 0.05 * close {
        CandlestickType::LongLowerShadow
    } else if upper_shadow > 0.05 * close {
        CandlestickType::LongUpperShadow
    }
    /*
    else if upper_shadow > 2.0 * body_length && lower_shadow > 2.0 * body_length {
        CandlestickType::ShootingStar
    } else {
        CandlestickType::SpinningTop
    }
     */
    else {
        CandlestickType::Unknown
    }
}
