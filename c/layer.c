#include <stdio.h>
#include <stdlib.h>
#include <math.h>

float WEIGHTS[5] = {-1.21721876, -0.80498737, -0.89646685, 1.30495203, -1.57395411};
float BIAS = 0.88239843;

float sigmoid(float x) {
    return 1 / (1 + exp((double) -x));
}

int main() {
    char *line = NULL; // Holds the line of characters
    char *start_p = NULL; //Holds the current starting position of the parser
    char *end_p = NULL; // Holds the end of the last parsed float
    float vec[5]; // Holds the input vector
    float temp = 0.0;

    // Read line store the pointer to that line in the char* line
    scanf ("%m[^\n]%*c", &line);

    // Parsing starts here
    start_p = line;
    int i = 0;

    while (*start_p) {
        // Read the next float and put it in the array. Update the end pointer
        temp = strtof(start_p, &end_p);
        if (start_p != end_p) {
            start_p = end_p + 1; // Update start_p to skip to the start of next float (end of prev + whitespace)
            vec[i] = temp;
            i++;
        } else {
            break;
        }
    }

    // vec should now contain the 5 floats
    float o = 0.0;
    for (int i = 0; i < 5; i++) {
        o += WEIGHTS[i] * vec[i];
    }
    o += BIAS;
    o = sigmoid(o);

    printf("Output: %f", o);

    return 0;
}