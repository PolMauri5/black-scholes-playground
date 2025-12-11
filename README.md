# Black-Scholes Playground

A minimal sandbox to experiment with the Black-Scholes option pricing model and observe how theoretical prices and Greeks respond to changes in the core inputs.

## What is the Black-Scholes Model?

Black-Scholes provides a closed-form solution for pricing European options under specific assumptions: lognormal asset dynamics, constant volatility, frictionless markets, and a risk-free rate driving discounting.  
Given a set of inputs, the model outputs a deterministic theoretical price for calls and puts, along with their sensitivities (Greeks).

## Required Inputs

To evaluate the formula, the following parameters are needed:

- **Spot** — current price of the underlying asset  
- **Strike** — option’s exercise price  
- **Interest Rate** — risk-free annual rate  
- **Implied Volatility (IV)** — expected volatility implied by the market  
- **Time to Expiration** — remaining time to maturity, expressed in years  
- **Option Type** — *call* or *put*

Each input alters either the drift, diffusion, or discounting mechanisms inside the model.

## Expected Output

The playground computes:

- **Theoretical Price** — Black-Scholes valuation for calls and puts  
- **Five Primary Greeks**:
  - **Delta** — sensitivity to spot changes  
  - **Gamma** — curvature of the price with respect to the spot  
  - **Vega** — sensitivity to volatility  
  - **Theta** — sensitivity to time decay  
  - **Rho** — sensitivity to interest-rate shifts
