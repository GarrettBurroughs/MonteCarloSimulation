# Monte Carlo Simulation

This is a monte carlo simulation that I wrote in Rust for APMA 3100: Probability. 

## Simulation Problem 

A representative of a high-speed Internet provider calls customers to asses their satisfaction with the service. It takes her 6 seconds to turn on a
phone and dial a number; then 3 additional seconds to detect a busy signal, or 25 additional seconds to wait for 5 rings and concloude that no one
will answer; and one second to end a call. After an unsuccessful call, she redials (in the course of several days) until the customer answers or she
has dialed four times. The *outcome* of each dialoing is determinde in an identical way: the customer being called is using the line with a
probability 0.2; or is unavailable to answer the call with probability with 0.3; or is available and can answer the call within *X* seconds, which is
a continous random variable with the mean of 12 seconds and the exponential distribution. The *calling process* ends when the customer answers the
call, or whne four unsuvvessful calls have been completed. 

## Report

To see a full report, you can see `Report.pdf`
