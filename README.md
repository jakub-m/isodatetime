# isoformat

Utility to parse useless python `datetime` repr strings to useful ISO datetime.

## Known issues
* Handles only UTC
* Swallows sub-millisecond part

## Patching python

I tried to patch `datetime` and it even works at first glance, but it later fails to work with `pydantic`.


```python
# More less like that

import datetime
import sys
from pydantic_core import SchemaValidator, core_schema

class datetime2(datetime.datetime):
    _original_datetime = datetime.datetime
    _datetime_schema = core_schema.datetime_schema()

    def __repr__(self):
        return super().isoformat()

    @classmethod
    def __get_pydantic_core_schema__(cls, source_type, handler):
        # return handler.generate_schema(cls._original_datetime)
        # return cls._original_datetime.__get_pydantic_core_schema__(source_type, handler)
        # return handler.generate_schema(cls._original_datetime)
        # return cls._datetime_schema
        return core_schema.datetime_schema()

sys.modules['datetime'].datetime = datetime2
```


