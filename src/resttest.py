import restpy as rp
import unittest


class TestStats(unittest.TestCase):

    def test_comb(self):
        self.assertEqual(rp.comb(12,3), 220)


if __name__ == '__main__':
    unittest.main()
