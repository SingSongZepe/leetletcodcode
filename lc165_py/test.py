import unittest
from main import *


class Test(unittest.TestCase):
    def test1(self):
        version1 = '1.2'
        version2 = '1.10'
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test2(self):
        version1 = '1.01'
        version2 = '1.001'
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test3(self):
        version1 = '1.0'
        version2 = '1.0.0.0'
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test4(self):
        version1 = '1.0'
        version2 = '1.0.0.1'
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test5(self):
        version1 = '0'
        version2 = '0'
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test6(self):
        version1 = "0599.9.083.970.70922.3.0.900.250.7.8.8.8.020.6.31586.7.0.1.4.2.5050800.3.590.34224.5.080.6.281.8649105.6.3.097.484.7.4.650.28907.7.090.200.3.455.6.5.7.695.932.6.0.460.3.6.712.117.08712.405.264.5.000.7.9.7.407.9.3.8.3.8.540.731.0381208.0.100.70900.401.101.012.30778.488.07874.002.0009040.9.4.0.8552202.757.0.2.9.830.9.04809.8.0.070840505.308.0.8.079.2.7.968.470.3.81002.2.8.19270.367.389.24378.5.5454000.4.6.739.5.309.0.0.4.10130.9.8.7673400.7.8.0804228.614090066.658.600.0.2.5.2.620.905.00503.519.0"
        version2 = "599.460.301.9524810.6.975.008.339.0.5.0.3.5.150.9"
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test7(self):
        version1 = "30307.038"
        version2 = "30307.038.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0"
        result = Solution().compareVersion(version1, version2)
        print(result)

    def test8(self):
        version1 = "7.5.2.4"
        version2 = "7.5.3"
        result = Solution().compareVersion(version1, version2)
        print(result)


    def test61(self):
        version1 = "0599.9.083.970.70922.3.0.900.250.7.8.8.8.020.6.31586.7.0.1.4.2.5050800.3.590.34224.5.080.6.281.8649105.6.3.097.484.7.4.650.28907.7.090.200.3.455.6.5.7.695.932.6.0.460.3.6.712.117.08712.405.264.5.000.7.9.7.407.9.3.8.3.8.540.731.0381208.0.100.70900.401.101.012.30778.488.07874.002.0009040.9.4.0.8552202.757.0.2.9.830.9.04809.8.0.070840505.308.0.8.079.2.7.968.470.3.81002.2.8.19270.367.389.24378.5.5454000.4.6.739.5.309.0.0.4.10130.9.8.7673400.7.8.0804228.614090066.658.600.0.2.5.2.620.905.00503.519.0"
        version2 = "599.460.301.9524810.6.975.008.339.0.5.0.3.5.150.9"
        result = Solution1().compareVersion(version1, version2)
        print(result)

    def test71(self):
        version1 = "30307.038"
        version2 = "30307.038.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0"
        result = Solution1().compareVersion(version1, version2)
        print(result)

    def test81(self):
        version1 = "7.5.2.4"
        version2 = "7.5.3"
        result = Solution1().compareVersion(version1, version2)
        print(result)


if __name__ == '__main__':
    unittest.main()