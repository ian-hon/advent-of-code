from enum import Enum

raw_data_string = """
[(<[(({{<{{[{({})}{<{}{}><()<>>}]}[{[({}{})]([{}()](<>{}))}[<<[]()]>[<[]<>>{[]()}]]]}{[[[{[][]}{[]()}]
(<<{((({<(<{([()[]]<[]<>>)}>({[{<>()}]<[<>()]({}())>})){{[(<<><>>{<>()})<{[]{}}>][{[[]()]<[]()>}
{({([({[[([<{(())([][])}<(<>())[[][]]>>{{{{}{}}{<>{}}}({{}<>}<<>()>)}]<<((<><>))<<[]>((){})>>[<{{}{}}
(<[{{{[[[<[[[([]<>)]{{()()}<()()>}][({()<>}(()()))[<[][]>[[]<>]]]]([{[(){}]}(<[]{}>{{}()))]<
{(<<({<{<<[[<[[]{}]>]<[<{}{}>[{}[]]]{[<>{})({})}>]><([[{(){}}{()[]}]])>><{({<[{}()]<{}[]>>(<
<<[[<<[({[<[{{[]()}[()()]}[<{}{}>[<>]]]><[[(<>[])]]{{{{}<>}[{}()]}{<()>[{}[]]}}>](({<((){}
<{[[((<{{[[(([(){}][()<>]))[(<()()>[[]<>]){[(){}][()[]]}]]]<([<[{}{}]([]<>)><<{}<>>{{}{}}>]<({[]{}}[{}{}]
<<<{<[[[[([<([[]<>])[[<>{}][[]<>]]>{{[{}{}]}[[[]()]]}][[<((){})<[]<>>>]<{[(){}][[]<>]}<<[][]](()())>>]){<((
[<{{({<<[{{<<(()())[{}[]]>{[<>{}]<[]<>>}>{[([]<>)[{}<>]](({}<>)[(){}])}}}[[(<{()}{<>()}>)<{<{}{}>([
<{<[{[[<[({{[[{}[]][()()>]((<><>)[[]{}])}{({{}{}}[()])<{<>{}}(<>[])>}}<<(<<>[]>[()])>>)]({{[([<>])(<{}[]>{
(({<{<{[[([<<{{}{}}>(<<>{}><()()>)>]{{(<{}>{{}[]})}(<([][]][<>[]]>{{<>{}}[<><>]})})(<[{{<>()}({}{})}{{{}[]}{<
{{[<<{{(({{{<{<>{}}{[]{}}>[<[]{}>{()()})}}(<({[]()}[()<>])[{<>()}<[]()>]>[[[{}[]][<>{}]]<{<
{{{<<<[(<[({([()()]))[<[()<>]{()()}>((()[])([]()))])][([<{{}()}<[]<>>>](([<>]<<>{}>){({}<>){[]<>
<<({{[([{{<[[<()()>{(){}}]{{()<>}[(){}]}]{[({}{})([]<>)]}>(<([{}()](()[]))({<>()}<<>[]>)>[<[()<>]<
{(<{{([{<{<(<({}{})([]())>)>{[{{{}[]}(<>{})}{([])<{}<>>}]<{<{}()><()[]>}<{[]{}}<[]>>>>}>}<(<<([
([([(<{<[(<[{<<><>>(()<>)}(([]{}))][<(<>{})<{}>>{[()[]](<><>)}]>[[[{[]{}}[[]{}]]]<{[()[]]}[[{}{}]<<><>>]>])
{<<({({{{[({[[<>[]]({}())]{[{}()]}}({[()()]{[]<>}}((()<>)<[]{}>))){[{{{}()}{{}()}}]}]}<<<[[<[]()>][
{{[[[{<<[(([{[<>{}]<<><>>}{{<>}{[]<>}}]<[<<>[]>({}{})]>)({<{()()}>{{()[]}({}())}})}]<([{[<[][]>{()[]}]}]
(<<[<[[[<{{[{{()}[{}[]]}[<{}<>>{[]{}}]]{{[()[]]<(){}>}}}(((<{}{}>{()()}))[<{<>()}{()<>}><[
[[{[<([([<<[[[()()]{()<>}]({<>{}}[<>[]])]>(<([<>()]<()<>>)[[{}()]]>)>])]){({{{[{<({}[]){[]()}><<[
(<[<[(([({{[({<>[]}[<>[]])[([]{})[<>[]]]]([<{}}[{}<>]][(()()){{}()}])}([<({}())[[]<>]>([[]{}]
{([[((<(<([{([<>[]]{()})[(()[])]}])(<[<{{}()}>]>)>)>)){{{<<[<[<(<><>){<>[]}>[[<>{}]]]({<<>{}>
<<(((({<<[{([({}{})[<>()]]<<(){}>({})>)}{<<{[]{}}{<><>}><{<>{}}[()[]]>}<(([][])(<>[]))<[<>[
{{{[{[[([({{{<{}{}>{()}}((()[]]<<>{}>)}}<[{[{}[]]<<>[]>}{[{}[]][()<>]}]<<({}[]){()()}><{<>{}}(<>[]
[[([(([({<[<([<>()][[]()])>{{<[]{}>[<>()]>[([]{})<(){}>]}]({[(()<>){{}()}]}<{(<><>)({}{})}{[[]<>]{{}()}}>
(<{([{{([<{<<<<>[]><[]()>>[(()[])[<>()]]>{({()<>}{(){}})<<{}<>>{<>{}}>}}[[{<{}()>}<(()<>)[()<>]>]{[[<>
<[[{({[<<<{{<[<>{}]({}<>)>(<()[]><[]>)}(<{{}()}<{}{}>>{{()}<[][]>})}[{({[]{}}(()[]))<[[]{}][[]{}]>}<{{<>{
{({[[<{<([<((({}{})[{}{}])){{[()<>]<<>()>}[(<><>){[]}]}><<<<()<>>(()<>)>>[{([]){<>{}}}]>]){({{[{{}
(<<[{[<<({<{<[<>[]][{}<>]>([{}<>]<<>()>)}{(<{}{}>{{}()})<([])>}>{(({[]())(<>[])){[[]<>][<><>]}
[({(((<(<<{({<()()>({}()>}((<>[])[[]]))}>{<(<({}<>)([]())>[{()<>}<<>>])<<([])<[]()>>{(<>{})[<>{}]}>>[[
<<{[(({<<{[{{<<>()>{<>{}}}({[]}{()})}][<[[[]()]]<<{}{}}[<>()]>>]}[<(([{}{}]<()<>>){{()<>}[[]<>]})<{<[
(<[[{{{<<(<<[<()[]>(()[])]<[()()]([][])>><{({}<>)[<><>]}>><[(({}){[][]})((()()))]>){{<[([]<>)[<><>]
[[[[<[<[[[<(([<>{}]<<>()>)(<<>()>{{}{}}))[([()()]([]<>))]>(<[([]())<[]<>>]{<{}{}>{{}{}}}){(<()[]
{[<(<[[<{<{[{({}[])[<>{}]}<[()<>]>][(<{}<>>(()[])){{()()}<()[]>}]}[<[(()[])]({{}{}}(<>{}))>]>}<{{[{([]{})<(){
(<[{{{{{{[{[{(<><>)({}<>)}[(()[]){<>[]}]]}[{([{}<>])}]]}((([{[[]][[]{}]}({<>()}[()[]])][({[
(<<<<[[{{<<((<{}()>[<><>]))<[<<><>>([][])]<{{}<>}([]<>)>}>{{[(<><>){<>{}}]{<<>()>(()())}}[<[<><>]{{}
{({[[<({<{[[[[[]<>](<>{})]((<>{})<[]()>)]([<()[]>({}<>)][[[]{}>[{}()]])]{({<[][]><[]()>}<(<>{})>)((({})
{[([[{<<[{<<[<()()>[{}[]]]>{(({}{})([]<>))[[{}()][()<>]]}>[<(([][])){[{}{}][{}{}]}>{{<(){}>([]<>)}
([{<{{{(<[{[{{[]}[()<>]}(<[]{}>{()<>})][(<[]><{}<>>)<([]{})(<>[])>]}][[{[<[]{}>][[<>[]]{<>[]}]}<((()[])(
<(({{<<[[<<[[(<>[]){[][]}]{({}())(()<>)}]{({{}<>})}><([<<>()><{}[]>][[<>{}]({}())])[(([][])({}<
[<<{{{[[[((<{([][])<<>()>}((<><>)([]{}))>))[<(<[<>()]><([][])<<><>>>)>[[<<{}{}>(<><>)><{[]{}}>}{({
<<{<<[[[(((({{[][]}[{}[]]}(<<>{}><{}<>>))[(([]<>)<()[]>)<[<><>]>]))<<(<<<>{}>>{(<>[])})<[<<
[<<{({{<[<[<<({}[])(<><>)>(([]{})([]<>))>]>]{[(<<{(){}}[[]<>]>>){([<{}()>[()()]](([]())[<><>]))
{({<[<[{(<{[([[]<>]{<>})<{<>[]}([])>]}<{((<>[])[{}[]}){{{}{}}<[]<>>}}<{[[]{}](()[])}<(()())
(({{<{<[[(<[([<>()][[]()])<({}<>)(()())>]{<<<>[]>{(){}}>}>([{({}())}{<[]>(())}]{<<[][]>({}[])>}))]
{{<{{(<[[[{<<<[][]><{}[]>><(())[{}[]]>>}][<(([[][]][<>{}])(<[][]>)){{([][])<{}{}>}[[[]{}]{[
[([{<[[<[{([{<[]{}>(<>{})}]<{[[]()]}[<()<>>]>)}][([[[([])<<>()>]{<{}<>>[()<>]}}((<{}{}>(()<>))(<{}
([<{(<({((<{<{()[]]{[]<>}>(<[][]>(()<>))}{<[()[]](()())>{([][])<<>>}}>))(<[(({[]()}(()[]))([(){}]{<>
([{{[{(<[((<<[[]()](()())>{<{}()}{[]}}>{(({}<>))<<<>{}>[{}()]>})[[((<>)<[]<>>)([()()]{<>{}})]<{[(){}]{[
<[<[<[[[({{{[[()<>]([]())]}}{({[()()]}{{<>[]}<<>{}>})[[{[]()}[{}{}]][{<>()}<{}{}>])}})<<{([{()[]}<[]
<([[<{<<{(<(<{[]}[{}()]>({()}{<>()})){{[[]()](<>[])}(<{}{}>(<><>))}><{{{(){}}<<><>>}}>){(([[()<>]<<><>
{(([(<[<{[[<{(()[])[{}<>]}<<{}()>>>([({}<>){<>{}}]([()[]]{{}()}))]({[[[][]]]})]}>][({[{<<([]<>){()<>}>({<><
([<(((<[<({<[({}{})((){})]{{()<>}<[]<>>}>{[{()[]}<<><>>]({{}<>}[[]<>])}})[[{([[][]](()[]))({()[]})}((((
{<[{[(([<(<{<[<>{}]>[((){})({}())]}{(<(){}>{[]<>})([()<>](()<>})}>[(<{[][]}{<>[]}>[<[]<>><<
[<<{((<(<((<<{[]<>}{[]<>}>{[<><>](()())}>{([<><>])[([]{})(<>[])]))(<<{[][]}{{}()}>[<<>{}>{{}{}}]><[({}{
(<([((({{([[<<{}()><()[]>>[{()<>}{{}{}}]]{<({}{})[[]<>]>{(()[]){<>}}}][{{({}{})}<([]<>)<()[]>>
({([[<<[[[[([<()()>{{}<>}]({{}[]}))]<{(<(){}>[()()])}>]{{<([<><>]{[]{}})<<{}<>>[()<>]>>}{[
<({{(<<((<<(<({}{})<{}()>>{{<>}})([[{}()]{{}()}][<<><>>])><{[<[]()>[()()]]<([]())>}>>[<<<{()}<
[[[(<((({{{{<{[][]}{<>{}}><{[]{}}<()[]>>}<([{}{}]{{}<>})>}<({[[]<>][[]<>]}[[()<>]<(){}>])>}{{(
(<<{({[{({[{(([]<>)({}[])}[<(){}>]}]})}][({(<<{(()())}[<(){}>(()[])]><({<>{}}<()()>){<[][]>[(){}]}>>
([({{{{{([{[{([][])<[][]>}<{()<>}<{}<>>>]}{[{([][])}<<()()>[{}[]]>]{<[[][]]{[]<>}>}}]<[(<<[]<
[([<({(<({[<{[{}<>]}>{[(()<>){<>()}]}](([([])[<>{}]])<[[()()]{[]}](((){}]{[]{}})>)}[<[<{<>[]
[({<[(<<{<[{[[(){}]<<><>>][<(){}>[{}[]]]}(({{}()}<[]<>>){([]()){{}()}})]><<(<<<>[]>{<>}>(({}()){<>()}))[[(<
{{{{<{{<<{[<([()[]][{}[]])[{<>{}}[(){}])><((()[]){()[]})[[{}[]](()<>)]>]}>>}}>{{[[{<(<(({}<>)[<>()]){[
([<{[<<[{({([<(){}>([])][(()<>)<<>()>])}{[([[][]]{{}()})<[{}]({}<>]>](([[]()]<{}<>>){<{}()>[[]()]})})[<
<((<{({[<(<(<<{}<>>{[]()}><<()<>>([][])>){<<[]{}>{<>[]}>{[<>[]][()()]}}>(({{()}{()<>}}({[]<>}(<>{})))
<[(<{{{<[(([{{[]<>}{{}()}}<[<><>]<<>[]>>]({((){})}))<(([<>()]<[][]>)(<()<>>([]<>)))>)]>[[[[[{[
(({{(([{{((<{<<><>><{}<>}}{{{}{}}{[]()}}>[[(<>[])[{}{}]]{<()[]>[[]{}]}])){{{{{()<>}<<><>>}{<[]<>>[[]<>]
(<{(((<<(({{{{[]()}}<<()()>{[]>>}[<{<>{}}{<><>}><{{}()}({}<>)>]}))(([((<(){}>({}()))<{<><>}<[]()>>)<(
<[[<[(<<{((<[<[]()>([]<>)]{(<>[])[<>()]}>[<(()<>){[]()}>{{[]<>}([][])}])[<(([]<>){<>()})([{}()]<[]{}>)>({
<<[[({<<({<{[({}[])<()<>>][[(){}]{[]()}]}[{<(){}>[<>[]]}{(<>[]>({}<>)}]><<([[]<>]({}))<{[]{}}(<><>
<(({([<<(({[{[[]<>]({}())}[{<><>}[[]()]]][(<{}()>)[{{}{}}[{}{}]]]}<{(<()<>>{()<>})<[()[]][{}{}]>}[[(<>[])
[<{<({{(({(<[{(){}})(<<>()><<>()>)>)<{<({}<>)[{}[]]>(<()()>(<><>))}{([<>[]][()<>])[{[]()}{[]}]}>}))<(
[<{<[[(<([<[<<{}<>>>{[<>[]]({}())}>{[(<>{})[(){}]]{[<>[]]}}>][{<{<[]()>{{}{}}}[[[]<>]<(){}>]>([<<><>>[{}{}
<<(({[<<{<([[{[]()}{<>()}]<[[][]]>][(((){}){<>{}})([{}()]<[]<>>)])>}>([[<<(<[]<>><()()>)<[()(
{<({[{<<<{[({<()[]>}[<<>()>[(){}]])<[(<>())<[][]>](<[]()>[{}<>])>]((<(<>{}){<>{}}>{{[]()}([])})[<{{}
[<([(<[{(<([<(<>[])[()()]>{[()<>][<><>]}]<[{{}[])]((<>())[[][]])>)>({(({<><>}({}[]))<{{}<>}([]<>)>)
{<[[({<[([{<{({}()){{}{}>}>{{[()[]]}[(<>[])<{}()>]}}])[[<(([<>]<{}<>>)([{}]{<>}))<({<><>}([]<>))<[<>[]][
{(<[[([[([{{[((){})(()<>)]}}[<(([]<>){{}{}})>{<[[]{}]{()()}>({{}()}[<>])}]][[(<[[]<>]<<><>>>
<{[<{{((<({{{[()[]]([]<>)}({{}<>}<{}()>)}(((()<>)[[]{}])[([]<>)<{}()>])]([[<()[]><<>()>]]<{{[][]}[[]]}>))>)<<
{([([<<((([(<([]<>){[][]}><{[]()}<<>{}>>)(<[<>()]([]<>)>[[[][]]<<>()>])]))(((<{<[]()><()<>>}([[]][()[]])>)<(
{<[(<<((<<({{[{}[]][<>[]]}[{{}<>}((){})]}<([()<>][{}[]])<{<>[]}({}())>>)(<[<(){}>]{{()[]}<()()>}>(<<()[]>
<(<{[({{<<{[({<>()}{<><>})(<{}{}>(<><>))]{<<{}<>>({}{})>}}[({(<>{})({}{})}{<{}{}><{}{}>})]}[<(
{<[([<(([([<(<()[]>(<>[]))[[()][<>()]]>[[{{}{}}(()())][<(){}><{}<>>]]]<((<()>((){}))<{{}{}}
((({{(({{[<[<{<>}<[]{}>>{([]{})<{}<>>}](<{<>{}}<<>()>><{(){}}[[]()]>)>({<([][]}<<>{}>><<{}()><{}{}>>}
[([{[<[[({[(<{<>[]}[{}[]]>(({}[]}{[]<>}))[<{<>{}}{<>()}>(([]<>))]][<(([])<{}()>)[<{}<>>{[]<
([<{(<[<[{([<[()[]]<()<>>>]{[{[]{}}<<>()>]}){[[[<><>][<>[]]]<({}[])<[]()>>](([[][]]<{}()>)[[<>{}]{()[]}])}}{
[[[([{{(({<[[<<>{}>(<>())][((){})]]]{((<()<>>{[][]})((<>())(()<>))){<<<><>>(<>[])>}}}(({<([]
({{[(([<(<([<{(){}}({}())><[{}<>]>]{(<<>{}><(){}>)})>){[<[<(()()}[<>{}]>((<><>)<<>()>)][([{}{}]){(<>[])[[
{([{((({(((([(()<>){[]()}](<{}[]>[{}{}]))<((()()){{}{}})[<[]()>]>)[((<<>[]>[()[]])(<{}<>><<
([[{[[[{<{[{{([]<>){()[]}}}]{({(()<>)({}{})}[<{}()>{[]{}}])<{[<>{}]}>}}>}]]]{<[({(<[<([]()){
((({[<<<{{(([({}<>)<[]<>>][{{}{}}])){([[()<>]]({(){}}[<><>]))((<()()>[{}<>]){{()<>}(()[])})}}<({((
<<<(<{(<([<{[[{}{}]]<{(){}}({}())>}[<<{}()><{}{}>><(()()){()<>}>]>]]>{<[([<<()<>>((){})>][[[()[]]<{}[]>]<(<>{
([{<[[[[[[{(<<()<>>([][]>>)({{()[]}<[][]>}((()()){()[]}))}((([{}<>]([]<>)){<<>()>{[][]}}))]<<[[[[]()][[][]]
{<<[[{{[([<<<{()<>}>[((){}){[]{}}]><({<>[]}[{}])([[][]]({}{}))>>[(([[]<>]){<()<>>[<>{}]}){<<(){}>({}{})>[{
<{(<(<([(<<([<[]<>>(<>{})][([]())<[][]>])([[<>()](()[])])>({(<{}<>>([][]))}[{[[][]]{(){}]}]
{[({{{[([{<((({}()){(){}})((<>[]){<>{}}))<<([][]){{}[]})[{{}{}}{{}[]}]>>}[{[[[<>[]]([]{})]<[{}()]<<><>>>](<((
[[{[(({<[[[({((){})<()<>>}{{<>{}}[()()]}){[([]{})([]<>)][{{}}<<>()>])][{([{}<>]({}())){([]{}){
((<[{(({<{<(((<>)<{}[]>)([<>[]]{()()}))>}>}[[<[{({[]{}}{[][]})}(<{{}[]}(()[])>(({}()){<>[]}))]<<([{}][()[]])>
<(({<[[<{{<<<[{}{}]{(){}}>{<()<>>[<>{}]}>>}[[{[([][])(()())]<[()<>]>}{<<(){}>[()[]]>([[]<>]<[][]>)}][{
<{((<({{({<<{([]<>]{<><>}}((<>[]){<><>})>>{<{[{}{}][<>[]]}>({{<><>}[[]]}<{(){}}[<>{}]>)}})}})<((<{[(<<()(
[(({({{([([{{<[]<>>([][])}<[()<>]>}{(({}())[<>()])}]<({{{}{}}}({{}<>}<[]{}>))>)(<[[<()>((){})]
"""


class Species(Enum):
    parenO = 0
    parenC = 1
    bracketO = 2
    bracketC = 3
    curlyO = 4
    curlyC = 5
    angleO = 6
    angleC = 7


class Status(Enum):
    Complete = 0
    Incomplete = 1
    Corrupted = 2


open_species = [
    Species.parenO,
    Species.bracketO,
    Species.curlyO,
    Species.angleO
]

closed_species = [
    Species.parenC,
    Species.bracketC,
    Species.curlyC,
    Species.angleC
]

species_match = {
    "(": Species.parenO,
    ")": Species.parenC,
    "[": Species.bracketO,
    "]": Species.bracketC,
    "{": Species.curlyO,
    "}": Species.curlyC,
    "<": Species.angleO,
    ">": Species.angleC
}

culprit_reward = {
    Species.parenC: 3,
    Species.bracketC: 57,
    Species.curlyC: 1197,
    Species.angleC: 25137
}

auto_complete_prize = {
    Species.parenC: 1,
    Species.bracketC: 2,
    Species.curlyC: 3,
    Species.angleC: 4
}


def species_matching(species1, species2):
    return species1.name.split(".")[-1][0:-1] == species2.name.split(".")[-1][0:-1]


def reverse_match(species):
    return [x for x in species_match if species_match[x] == species][0]


def inverse_type(species):
    similiar_species = [x for x in Species if species.name[0:-1] == x.name.split('.')[-1][0:-1]]
    return [x for x in similiar_species if x.name[-1] != species.name[-1]][0]


class Line:
    lines = []

    def __init__(self, string):
        self.line_string = string
        self.line_data = [species_match[x] for x in self.line_string]
        self.status = Status.Complete
        self.culprit = ''
        self.auto_complete_species = []
        self.auto_complete_reward = 0

        self.to_be_closed = []
        for species in self.line_data:
            break_later = False
            if species in open_species:
                self.to_be_closed.append(species)
            else:
                if (self.to_be_closed[-1] in open_species) and species_matching(self.to_be_closed[-1], species):
                    self.to_be_closed.pop(-1)
                else:
                    self.status = Status.Corrupted
                    self.culprit = species
                    break_later = True

            # print(f"{reverse_match(species)} : {''.join([reverse_match(x) for x in to_be_closed])}")

            if break_later:
                break

        if self.status != Status.Corrupted:
            if len(self.to_be_closed) == 0:
                self.status = Status.Complete
            else:
                self.status = Status.Incomplete

        # print(f"{self.status} {len(to_be_closed)} : {''.join([reverse_match(x) for x in to_be_closed])}")

    def auto_complete(self):
        if self.status == Status.Corrupted:
            return None
        self.auto_complete_species = []
        while len(self.to_be_closed) != 0:
            candidate = inverse_type(self.to_be_closed[-1])
            self.auto_complete_species.append(candidate)
            self.to_be_closed.pop(-1)

        # return sum([auto_complete_prize[x] for x in self.auto_complete_species])
        for x in self.auto_complete_species:
            self.auto_complete_reward *= 5
            self.auto_complete_reward += auto_complete_prize[x]


for x in raw_data_string.strip().split("\n"):
    Line.lines.append(Line(x))

for x in Line.lines:
    if x.status != Status.Incomplete:
        continue
    x.auto_complete()

auto_complete_rewards = [x.auto_complete_reward for x in Line.lines if x.auto_complete_reward != 0]
auto_complete_rewards.sort()
middle_index = int(len(auto_complete_rewards)/2)
print(auto_complete_rewards[middle_index])
