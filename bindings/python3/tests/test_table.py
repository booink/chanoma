import unittest
from chanoma import Table, Chanoma, Item, Correspondence
import chanoma

class TestTable(unittest.TestCase):
    def test_empty(self):
        table = Table()
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ＡＢＣＤＥ')

    def test_preset(self):
        table = Table(preset=True)
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')

    def test_from_csv(self):
        table = Table(csv="./tests/test.csv")
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')

    def test_from_yaml(self):
        table = Table(yaml="./tests/test.yaml")
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')

    def test_add_alphabets(self):
        table = Table()
        table.add(chanoma.characters_set.Alphabets())
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')

    def test_add_correspondence(self):
        corr = Correspondence([ Item("Ａ", "A"), Item("Ｂ", "B"), Item("Ｃ", "C"), Item("Ｄ", "D"), Item("Ｅ", "E") ])
        table = Table()
        table.add(corr)
        c = Chanoma(table=table)
        self.assertEqual(c.normalize("ＡＢＣＤＥ"), 'ABCDE')
