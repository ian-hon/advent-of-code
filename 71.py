raw_data_string = """1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,51,812,37,278,203,12,1699,10,24,482,200,197,433,141,1854,148,529,748,46,1366,41,329,300,29,159,767,661,238,586,940,139,1606,273,1093,1687,694,232,1069,264,162,752,250,138,471,828,72,285,136,817,258,586,308,191,478,43,750,570,38,207,1221,434,124,1410,125,743,7,827,963,873,263,478,938,686,250,1022,7,917,717,1354,618,639,24,113,417,550,279,919,736,75,117,1173,32,172,88,1435,15,442,232,272,102,253,113,173,86,57,536,1282,111,18,197,117,738,427,910,740,861,90,706,520,8,1,129,80,79,36,788,1545,119,971,1435,945,808,821,1080,227,1257,973,39,303,818,669,7,197,819,1683,50,2,1248,1459,669,210,653,978,76,509,173,304,183,228,45,1032,672,792,12,540,839,135,153,55,29,1190,42,395,626,487,54,831,956,1,1012,1461,929,561,34,733,629,49,146,469,220,1368,89,265,128,521,402,557,1121,853,240,655,100,341,137,525,371,288,1389,430,1148,398,130,174,176,982,74,923,1438,469,572,33,261,126,456,300,174,27,60,1052,428,196,403,394,392,40,474,27,351,194,619,657,722,181,300,448,1037,525,1388,854,1459,967,211,46,1708,1175,1225,613,1315,479,973,573,324,887,2,116,752,447,3,1074,1135,72,595,601,632,511,1349,267,164,6,1300,172,412,3,298,1120,93,161,176,141,150,67,37,144,421,45,1451,781,1120,205,487,344,372,150,136,614,265,536,1740,265,1367,0,322,204,76,97,1112,717,444,418,279,943,597,309,322,205,1167,292,18,383,367,621,770,13,243,1641,500,313,785,106,184,310,615,248,664,98,221,740,450,460,7,23,1226,183,75,449,806,721,1057,266,254,1083,0,125,27,151,16,664,73,94,44,1347,73,325,958,475,862,1096,1523,114,307,1418,46,113,188,462,194,535,282,1144,26,1106,1465,39,133,445,177,481,233,696,181,72,1466,747,266,44,311,1061,505,140,956,360,716,98,844,1059,305,162,1679,817,873,969,793,1079,320,318,70,417,1170,628,1628,1515,894,482,1757,423,1024,267,1280,10,474,806,684,378,425,816,243,388,27,116,569,777,946,593,646,91,639,508,63,405,1310,639,380,323,75,860,67,42,58,198,35,58,180,75,530,25,194,1743,476,1092,795,243,121,1326,409,1300,218,1393,371,64,412,209,255,648,480,71,125,1398,45,1035,1245,1426,1765,596,187,353,0,261,774,958,1303,397,1024,1076,1225,307,69,789,307,450,143,203,259,21,2,297,963,1236,1292,595,784,100,1194,1246,1820,534,58,244,12,194,1316,211,368,192,741,1232,23,87,551,291,12,512,6,42,1513,619,62,1339,375,743,137,1486,254,53,274,299,1443,844,899,753,414,241,161,52,163,66,86,503,823,528,150,376,403,1346,125,363,412,774,374,1090,1001,177,1379,74,193,49,92,294,679,108,228,199,1203,324,64,321,89,601,32,46,1274,519,1089,1107,63,379,1062,1034,129,736,716,156,526,445,1,299,388,444,1080,1016,101,735,315,517,13,390,537,155,140,1119,975,259,254,402,277,1160,372,55,392,1022,1119,4,735,266,260,1550,389,824,1426,23,65,480,151,176,1761,0,16,139,152,383,358,1155,95,1138,310,232,71,1073,22,1,335,1168,792,136,902,33,204,59,146,1063,1012,103,1083,160,885,445,499,473,278,451,191,1940,249,37,722,325,495,615,70,85,50,107,560,597,75,206,767,990,113,530,94,1343,250,116,67,417,390,500,633,736,132,473,646,1502,249,119,228,3,64,212,19,1005,324,14,418,619,847,20,878,533,204,49,820,216,34,60,62,119,680,88,359,8,473,882,138,387,297,419,664,693,420,101,53,829,3,101,272,726,639,368,363,0,33,70,0,626,525,364,784,271,73,536,318,598,794,34,314,1248,1596,764,34,202,1383,635,158,1095,76,0,119,176,1158,301,409,796,242,1765,808,59,0,278,4,8,359,1111,818,931,220,109,292,353,532,750,333,223,725,1476,199,1,201,55,72,117,37,210,400,108,619,863,187,372,15,574,380,635,332,1,1210,64,897,501,12,822,508,250,263,1044,72,15,210,901,219,471,292,179,572,733,422,1354,1197,202,538,662,261,973,0,465,522,412,9,166,325,237,757,115,1046,273,549,174,30,96,215,113,7,1032,671,262,202,332,1078,629,555,26,8,29,349,206,123,1093,673,1356,513,1454,518,1240,337,96,115,1160,17,331,1450,114,107,782,995,168"""


class Crab:
    crabs = []

    def __init__(self, pos):
        self.position = pos

    def distance(self, target):
        return abs(self.position - target)


for x in raw_data_string.split(","):
    if not x:
        continue
    Crab.crabs.append(Crab(int(x)))


# range = 0 - 1940
results = []
for index in range(2000):
    final = 0
    for crab in Crab.crabs:
        final += crab.distance(index)
    results.append(final)

minimum = min(results)
print(minimum)
print(results.index(minimum))

