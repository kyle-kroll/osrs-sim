# osrs-sim
A drop simulator for Old School RuneScape

## Usage
After compilation use the following format to run this program:
`./osrs-sim --iterations [INT] --droprate [DECIMAL]` 

For example to run 100,000 iterations of a 1/5000 (0.0002) droprate you would run the following:

`./osrs-sim --iteration 100000 --droprate 0.0002`

## Plans
1. Optimize further to reduce run times. Current runtimes on a 2019 macbook air for 1,000,000 iterations of 0.0002 droprate take ~7s. 
2. Build releases for major systems
3. Convert to WASM and build web front end.
