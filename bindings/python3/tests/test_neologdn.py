import unittest
from chanoma import Table, Chanoma, Item, Correspondence
import chanoma

class TestNeologdn(unittest.TestCase):
    def test_neologdn(self):
        c = Chanoma(neologdn=True)
        self.assertEqual(c.normalize("０１２３４５６７８９"), "0123456789")
        self.assertEqual(c.normalize("ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ"), "ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        self.assertEqual(c.normalize("ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ"), "abcdefghijklmnopqrstuvwxyz")
        self.assertEqual(c.normalize("！”＃＄％＆’（）＊＋，－．／：；＜＞？＠［￥］＾＿｀｛｜｝"), "!\"#$%&'()*+,-./:;<>?@[¥]^_`{|}")
        self.assertEqual(c.normalize("＝。、・「」"), "＝。、・「」")
        self.assertEqual(c.normalize("ﾊﾝｶｸ"), "ハンカク")
        self.assertEqual(c.normalize("o₋o"), "o-o")
        self.assertEqual(c.normalize("majika━"), "majikaー")
        self.assertEqual(c.normalize("わ〰い"), "わい")
        self.assertEqual(c.normalize("スーパーーーー"), "スーパー")
        self.assertEqual(c.normalize("!#"), "!#")
        self.assertEqual(c.normalize("ゼンカク　スペース"), "ゼンカクスペース")
        self.assertEqual(c.normalize("お             お"), "おお")
        self.assertEqual(c.normalize("      おお"), "おお")
        self.assertEqual(c.normalize("おお      "), "おお")
        self.assertEqual(c.normalize("検索 エンジン 自作 入門 を 買い ました!!!"), "検索エンジン自作入門を買いました!!!")
        self.assertEqual(c.normalize("アルゴリズム C"), "アルゴリズムC")
        self.assertEqual(c.normalize("　　　ＰＲＭＬ　　副　読　本　　　"), "PRML副読本")
        self.assertEqual(c.normalize("Coding the Matrix"), "Coding the Matrix")
        self.assertEqual(c.normalize("南アルプスの　天然水　Ｓｐａｒｋｉｎｇ　Ｌｅｍｏｎ　レモン一絞り"), "南アルプスの天然水Sparking Lemonレモン一絞り")
        self.assertEqual(c.normalize("南アルプスの　天然水-　Ｓｐａｒｋｉｎｇ*　Ｌｅｍｏｎ+　レモン一絞り"), "南アルプスの天然水-Sparking*Lemon+レモン一絞り")
