#include <stdio.h>
#include <stdlib.h>
#include <time.h>

size_t throwDice(const size_t nSides) {
	return rand() % (int)nSides;
}

size_t gameOn(size_t* fishPos, const size_t nFish, size_t manPos, const size_t nSides, const size_t strategy) {
	size_t fishInTheGame = nFish;
	size_t fishEscaped = 0;
	size_t side, ii, jj;
	size_t* pos;
	while (fishInTheGame > 0) {
		// throw the dice
		side = throwDice(nSides);
		if (side >= nFish || (strategy && fishPos[side] >= manPos)) {
			// the boat is coming closer
			manPos--;
			for (ii = 0; ii < nFish; ii++) {
				if (fishPos[ii] == manPos) {
					// fish caught!!! :(
					fishInTheGame--;
				}
			}
		} else {
			// the fish escape
			pos = &fishPos[side];
			size_t moveAndCheck = 0;
			if (*pos > 0 && *pos < manPos) {
				moveAndCheck = 1;
			} else if (strategy && *pos == 0) {
				size_t posMax = 0;
				for (ii = 0; ii < nFish; ii++) {
					if (fishPos[ii] > posMax) {
						pos = &fishPos[ii];
					}
				}
			}
			if (moveAndCheck) {
				*pos = *pos - 1;
				if (*pos == 0) {
					// fish reached the sea!!! :)
					fishEscaped++;
					fishInTheGame--;
				}
			}
		}
	}
	return fishEscaped;
}

int main( int argc, const char* argv[] )
{
	// number of games
	size_t N = 1;
	size_t newStrategy = 0;
	if (argc > 1) {
		N = atoi(argv[1]);
	} 
	if (argc > 2) {
		newStrategy = atoi(argv[2]);
	}
	// dimensions
	const size_t nFish = 4;
	const size_t nMan  = 2;
	const size_t dof   = nMan + nFish;
	const size_t fishStart = 5;
	const size_t manStart  = 11;
	// init stuff
	srand((unsigned int)time(NULL));
	size_t position[nFish];
	// start games
	size_t fishEscaped[N];
	size_t count[nFish+1];
	for (size_t jj = 0; jj < N; jj++) {
		// init game
		for (size_t ii = 0; ii < nFish; ii++) {
			position[ii] = fishStart;
		}
		fishEscaped[jj] = gameOn(position, nFish, manStart, dof, newStrategy);
		count[fishEscaped[jj]]++;
	}
	// print result
	size_t sum = 0;
	for (size_t ii = 0; ii <= nFish; ii++) {
		printf(" %i: %f\n", ii, (double)count[ii]/(double)N);
		sum += (ii * count[ii]);
	}
	printf("average escapes: %f\n", (double)sum/(double)N);
	// return success
	return 0;
}