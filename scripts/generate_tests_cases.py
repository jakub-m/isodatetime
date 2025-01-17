from datetime import datetime, timedelta, timezone


def main():
    t0 = datetime.fromtimestamp(0, timezone.utc)
    for d in iter_deltas():
        t = t0 + d
        print(f"{t!r}\t{isoformat_z(t)}")
    for s in iter_datetime_str():
        print(f"{s}")


def isoformat_z(d: datetime) -> str:
    return d.isoformat().replace("+00:00", "Z")


def iter_deltas():
    yield timedelta(0)
    yield timedelta(milliseconds=1)
    yield timedelta(milliseconds=-1)

def iter_datetime_str():
    yield 'datetime.datetime(2024, 8, 13, 16, 11, tzinfo=<UTC>)'

main()
