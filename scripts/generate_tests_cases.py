from datetime import datetime, timedelta, timezone


def main():
    t0 = datetime.fromtimestamp(0, timezone.utc)
    for d in iter_deltas():
        t = t0 + d
        print(f"{t!r}\t{isoformat_z(t)}")


def isoformat_z(d: datetime) -> str:
    return d.isoformat().replace("+00:00", "Z")


def iter_deltas():
    yield timedelta(0)
    yield timedelta(milliseconds=1)
    yield timedelta(milliseconds=-1)


main()
