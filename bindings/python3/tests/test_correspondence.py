import unittest
from chanoma import Table, Chanoma, Item, Correspondence
import chanoma

class TestCorrespondence(unittest.TestCase):
    def test_empty_item(self):
        corr = Correspondence([])
        self.assertEqual(len(corr.items), 0)

    def test_one_item(self):
        item = Item("A", "a")
        corr = Correspondence([item])
        self.assertEqual(len(corr.items), 1)
