#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 1000

int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

int load_inputs(const char *filename, int *list_one, int *list_two,
                int max_size, int *rows_loaded) {
    FILE *file;
    int i = 0;

    file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file");
        return -1;
    }

    while (fscanf(file, "%d %d", &list_one[i], &list_two[i]) == 2 &&
           i < max_size) {
        i++;
    }

    fclose(file);
    *rows_loaded = i;
    return i;
}

int calculate_total_distance(int *list_one, int *list_two, int rows) {
    qsort(list_one, rows, sizeof(int), compare);
    qsort(list_two, rows, sizeof(int), compare);

    int total_distance = 0;
    for (int i = 0; i < rows; i++) {
        total_distance += abs(list_one[i] - list_two[i]);
    }

    return total_distance;
}

int calculate_total_similarity(int *list_one, int *list_two, int rows) {
    int total_similarity = 0;

    for (int i = 0; i < rows; i++) {
        int count = 0;
        for (int j = 0; j < rows; j++) {
            if (list_one[i] == list_two[j]) {
                count++;
            }
        }
        total_similarity += list_one[i] * count;
    }

    return total_similarity;
}

int main() {
    int list_one[MAX_SIZE], list_two[MAX_SIZE];
    int rows_loaded = 0;

    load_inputs("../inputs/day_one.txt", list_one, list_two, MAX_SIZE, &rows_loaded);

    if (rows_loaded < 0) {
        fprintf(stderr, "Failed to load data from file.\n");
        return 1;
    }

    int total_distance = calculate_total_distance(list_one, list_two, rows_loaded);
    printf("The Total Distance is %d\n", total_distance);

    int total_similarity = calculate_total_similarity(list_one, list_two, rows_loaded);
    printf("The Toal Similarity is %d\n", total_similarity);

    return 0;
}
