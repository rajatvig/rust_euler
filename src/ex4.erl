-module(ex4).
-export([palindrome/1]).

palindrome(N) ->
    Reversed = lists:foldr(fun(X, Acc) -> (Acc * 10) + X end, 0, p1(N, [])),
    Reversed == N.

p1(N, Prev) when N == 0 ->
    Prev;
p1(N, Prev) when N > 0 ->
    Next = N rem 10,
    p1(N div 10, [Next | Prev]).
