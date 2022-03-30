import unittest
from chanoma import Table, Chanoma, Item, Correspondence
import os, shutil

class TestChanoma(unittest.TestCase):
    def test_empty_table(self):
        c = Chanoma()
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ＡＢＣＤＥ')

    def test_env_rc(self):
        os.environ['CHANOMARC'] = './tests/test.csv'
        c = Chanoma(rc=True)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')

    def test_rc(self):
        rc_file = "./.chanomarc.csv"
        shutil.copyfile("./tests/test.csv", rc_file)
        c = Chanoma(rc=True)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')
        os.remove(rc_file)
