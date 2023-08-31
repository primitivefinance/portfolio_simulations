# trading function

`k = Φ⁻¹(y/K) - Φ⁻¹(1-x) + σ√τ`


# Special Boundary Properties

The Φ⁻¹(y/K) term is denoted as `invariantTermY`
The Φ⁻¹(1-x) term is denoted as `invariantTermX`.

As y approaches its upper bound of K, Φ⁻¹(y/K) approaches +∞.
As y approaches its lower bound of 0, Φ⁻¹(y/K) approaches -∞.
As x approaches its upper bound of 1, Φ⁻¹(1-x) approaches -∞.
As x approaches its lower bound of 0, Φ⁻¹(1-x) approaches +∞.

The maximum output of Φ⁻¹(1E18 - 1) = 8710427241990476442 [~8.71e18]. In wad units.
The minimum output of Φ⁻¹(0 + 1)   = -8710427241990476442 [~-8.71e18]. In wad units.


In the sim, y -> inf, x -> 0

Φ⁻¹(1) - Φ⁻¹(1)


 k = 4633509197977427441

 k = Φ⁻¹(y/K) - Φ⁻¹(1-x) + σ√τ
 k = Φ⁻¹(y/K) - 8710427241990476442 + σ√τ
 k = Φ⁻¹(y/K) - 8710427241990476442 + 1e14
 4633509197977427441 = Φ⁻¹(y/K) - 8710427241990476442 + 1e14
 4633509197977427441 + 8710427241990476442 - 1e14 = Φ⁻¹(y/K)
 13343836439967903883 = Φ⁻¹(y/K)
 Φ(13343836439967903883) = y / K
 1E18 = y / K


 Actually:

 k = 8.710427241990476442 - 2.83769359353855866095694985651 + σ√τ


previous invariant 214289665497284135
invariant in csv: 4633509197977427441
https://keisan.casio.com/calculator
Post invariant - prev invariant
(normalicdlower(0.999999999999999999) - normalicdlower(1 - 0.002272039048866105 / 100) + 0.0001) - (normalicdlower(83.181852570270892200 / 100) - normalicdlower(1 - 22.720390488661045400 / 100) + 0.0001)
 = 4.46608271699689934281522507575 increase in invariant

 0.16742648098052809818477492425 disprecency

disprecency is the fee amount of the trade, 5 bips, seems close



