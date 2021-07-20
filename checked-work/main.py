from iex_parser import Parser, TOPS_1_6
from dateutil.parser import parse

d = parse('2021/07/12 09:35:00 -0400')

TOPS_SAMPLE_DATA_FILE = 'data_feeds_20210712_20210712_IEXTP1_TOPS1.6.pcap.gz'

with Parser(TOPS_SAMPLE_DATA_FILE, TOPS_1_6) as reader:
    for message in reader:
      if (message['timestamp'] > d):
        print('DONE')

      if (
        (
          message['type'] == 'quote_update' or
          message['type'] == 'trade_report'
        ) and
        message['symbol'] == b'NET' and
        message['timestamp'] < d
      ):
        print(message)
