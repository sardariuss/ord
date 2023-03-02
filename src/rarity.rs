use super::*;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Rarity {
  Common,
  Uncommon,
  Rare,
  Epic,
  Legendary,
  Mythic,
}

impl Display for Rarity {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Common => "common",
        Self::Uncommon => "uncommon",
        Self::Rare => "rare",
        Self::Epic => "epic",
        Self::Legendary => "legendary",
        Self::Mythic => "mythic",
      }
    )
  }
}

impl From<Sat> for Rarity {
  fn from(sat: Sat) -> Self {
    let Degree {
      hour,
      minute,
      second,
      third,
    } = sat.degree();

    if hour == 0 && minute == 0 && second == 0 && third == 0 {
      Self::Mythic
    } else if minute == 0 && second == 0 && third == 0 {
      Self::Legendary
    } else if minute == 0 && third == 0 {
      Self::Epic
    } else if second == 0 && third == 0 {
      Self::Rare
    } else if third == 0 {
      Self::Uncommon
    } else {
      Self::Common
    }
  }
}

impl FromStr for Rarity {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "common" => Ok(Self::Common),
      "uncommon" => Ok(Self::Uncommon),
      "rare" => Ok(Self::Rare),
      "epic" => Ok(Self::Epic),
      "legendary" => Ok(Self::Legendary),
      "mythic" => Ok(Self::Mythic),
      _ => Err(anyhow!("invalid rarity: {s}")),
    }
  }
}

impl Serialize for Rarity {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_str(self)
  }
}

impl<'de> Deserialize<'de> for Rarity {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(DeserializeFromStr::deserialize(deserializer)?.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rarity() {
    assert_eq!(Sat(0).rarity(), Rarity::Mythic);
    assert_eq!(Sat(1).rarity(), Rarity::Common);

    assert_eq!(Sat(50 * COIN_VALUE - 1).rarity(), Rarity::Common);
    assert_eq!(Sat(50 * COIN_VALUE).rarity(), Rarity::Uncommon);
    assert_eq!(Sat(50 * COIN_VALUE + 1).rarity(), Rarity::Common);

    assert_eq!(
      Sat(50 * COIN_VALUE * DIFFCHANGE_INTERVAL - 1).rarity(),
      Rarity::Common
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * DIFFCHANGE_INTERVAL).rarity(),
      Rarity::Rare
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * DIFFCHANGE_INTERVAL + 1).rarity(),
      Rarity::Common
    );

    assert_eq!(
      Sat(50 * COIN_VALUE * SUBSIDY_HALVING_INTERVAL - 1).rarity(),
      Rarity::Common
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * SUBSIDY_HALVING_INTERVAL).rarity(),
      Rarity::Epic
    );
    assert_eq!(
      Sat(50 * COIN_VALUE * SUBSIDY_HALVING_INTERVAL + 1).rarity(),
      Rarity::Common
    );

    assert_eq!(Sat(2067187500000000 - 1).rarity(), Rarity::Common);
    assert_eq!(Sat(2067187500000000).rarity(), Rarity::Legendary);
    assert_eq!(Sat(2067187500000000 + 1).rarity(), Rarity::Common);
  }

  #[test]
  fn from_str_and_deserialize_ok() {
    #[track_caller]
    fn case(s: &str, expected: Rarity) {
      let actual = s.parse::<Rarity>().unwrap();
      assert_eq!(actual, expected);
      let round_trip = actual.to_string().parse::<Rarity>().unwrap();
      assert_eq!(round_trip, expected);
      let serialized = serde_json::to_string(&expected).unwrap();
      assert!(serde_json::from_str::<Rarity>(&serialized).is_ok());
    }

    case("common", Rarity::Common);
    case("uncommon", Rarity::Uncommon);
    case("rare", Rarity::Rare);
    case("epic", Rarity::Epic);
    case("legendary", Rarity::Legendary);
    case("mythic", Rarity::Mythic);
  }

  #[test]
  fn from_str_err() {
    "abc".parse::<Rarity>().unwrap_err();

    "".parse::<Rarity>().unwrap_err();
  }
}
