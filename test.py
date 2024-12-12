data = """337 42493 1891760 351136 2 6932 73 0"""

# evaluate = lambda t, c, a: [a := [a + nested_i, a * nested_i][format(t, 'b').rjust(len(c) - 1, '0')[nested_index] == '0'] for (nested_index, nested_i) in enumerate(c[1::])][-1]
# print(sum([(lambda t, o: max([evaluate(i, o, o[0]) if (evaluate(i, o, o[0]) == t) else 0 for i in range(0, pow(2, len(o) - 1))]))(int(l.split(':')[0]), [int(item) for item in l.split(':')[1].split()]) for l in data.strip().split('\n')]))

# print(sum([(lambda t, o: max([(lambda t, c, a: [a := [a + nested_i, a * nested_i][format(t, 'b').rjust(len(c) - 1, '0')[nested_index] == '0'] for (nested_index, nested_i) in enumerate(c[1::])][-1])(i, o, o[0]) if ((lambda t, c, a: [a := [a + nested_i, a * nested_i][format(t, 'b').rjust(len(c) - 1, '0')[nested_index] == '0'] for (nested_index, nested_i) in enumerate(c[1::])][-1])(i, o, o[0]) == t) else 0 for i in range(0, pow(2, len(o) - 1))]))(int(l.split(':')[0]), [int(item) for item in l.split(':')[1].split()]) for l in data.strip().split('\n')]))

# v = 'xxxx'

# result = 0
# l = 0
# for r, i in enumerate(v):
#     current = v[l:r]
#     if len(set(current)) != len(current):
#         l = r
#         result = max(result, len(set(current)))
# result = max(result, len(set(current)))
# print(result)

# m = {}
# for d in data.split():
#     m[d] = 1
# a = {67: 1, 2457: 1, 45: 1, 9456: 1, 3686: 1, 56: 2, 13: 1, 8096: 8, 40: 2, 3277: 3, 94: 4, 20: 3, 7: 1, 96: 3, 12144: 2, 3: 1, 4048: 1, 0: 7, 2608: 3, 24: 5, 48: 2, 57: 2, 4: 2, 98: 1, 8: 6, 16192: 5, 2024: 2, 6: 6, 42: 1, 77: 1, 14168: 1, 9: 4, 2: 6, 1: 6, 60: 1, 76: 1, 88: 1, 80: 3, 26: 1, 32772608: 1, 9184: 1, 28: 1, 18216: 1, 32: 2}
# b = {1: 6, 8096: 8, 16192: 5, 12144: 2, 18216: 1, 2: 6, 0: 7, 4: 2, 98: 1, 88: 1, 94: 3, 76: 1, 13: 1, 42: 1, 45: 1, 2024: 2, 96: 3, 80: 3, 32772608: 1, 6: 6, 7: 1, 8: 6, 3: 1, 40: 2, 48: 2, 9: 4, 20: 3, 24: 5, 2608: 3, 3277: 3, 2457: 1, 9456: 1, 9184: 1, 3686: 1, 4048: 1, 14168: 1, 57: 2, 56: 2, 32: 2, 60: 1, 28: 1, 67: 1, 26: 1, 77: 1}

# print(sorted([(k, v) for k, v in a.items()]))
# print()
# print(sorted([(k, v) for k, v in b.items()]))
# quit()

data = "125 17"

m = { int(x) : 1 for x in data.split() }
def insert(k, v, n):
    # print(l, v)
    # for k in list(set(list(n.keys()) + l)):
    # # for k in l:
    #     if k in l:
    #         n[k] = v + (n[k] if k in n else 0)
    # return n
    
    # this works, but its two lines
    # n[k] = v + (n[k] if k in n else 0)
    # return n
    return {
        # this is broken
        key:(n[key] if key in n else v)
        for key in list(set(list(n.keys()))) + [k]
    }
    
    # return {
    #     k:(n[k] + v if (k in n) else v)
    #     for k in list(set([k] + list(n.keys())))
    # }


    # return {
    #     k:((n[k] + v if (k in n) else v) if (k in l) else n[k])
    #     for k in list(set(l + list(n.keys())))
    # }

# l = [18]
# print(list(set(list(m.keys()) + l)))
# for k in list(set(list(m.keys()) + l)):
#     if k in l:
#         # print(k)
#         pass
#     else:
#         print(k)
# quit()

for index in range(0, 25):
    # [1] if k == 0 else ([int(str(k)[0:(int(len(str(k)) / 2))]), int(str(k)[(int(len(str(k)) / 2)):len(str(k))])] if ((len(str(k)) % 2) == 0) else ([k * 2024])),
    n = {}
    # m = {
    #     ii[0]: ii[1]
    #     for i in 
    #     [
    #         insert(
    #             [1] if k == 0 else ([int(str(k)[0:(int(len(str(k)) / 2))]), int(str(k)[(int(len(str(k)) / 2)):len(str(k))])] if ((len(str(k)) % 2) == 0) else ([k * 2024])),
    #             v, n
    #         )
    #         for k, v in m.items()
    #     ]
    #     for ii in i
    # }
    m = [
        n := [
            insert(x, v, n) for x in
            (
                [1] if k == 0 else
                (
                    [
                        int(str(k)[0:(int(len(str(k)) / 2))]),
                        int(str(k)[(int(len(str(k)) / 2)):len(str(k))])
                    ] if ((len(str(k)) % 2) == 0) else ([k * 2024])
                )
            )
        ][-1]
        for k, v in m.items()
    ][-1]
    
    print(m)
    print('\n')


print(sum([v for v in m.values()]))
