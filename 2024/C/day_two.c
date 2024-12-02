#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_REPORTS 1000
#define MAX_LEVELS 8
#define MAX_LINE_LENGTH 256

int load_inputs(int reports[][MAX_LEVELS], int *rows_loaded, int *counts) {
    FILE *file;
    int i = 0;

    file = fopen("../inputs/day_two.txt", "r");
    if (file == NULL) {
        perror("Error opening file");
        return -1;
    }

    char line[MAX_LINE_LENGTH];

    int report_index = 0;
    while (fgets(line, sizeof(line), file) != NULL && report_index < MAX_REPORTS) {
        char *token = strtok(line, " \t\n");  // Split by space, tab, or newline
        int level_index = 0;

        while (token != NULL && level_index < MAX_LEVELS) {
            reports[report_index][level_index] = atoi(token);  // Convert to int
            token = strtok(NULL, " \t\n");
            level_index++;
        }
        counts[report_index] = level_index;  // Store the count of numbers in this line
        report_index++;
    }

    fclose(file);
    *rows_loaded = i;
    return i;
}

int is_report_safe(int report[], int length) {
    int zero = 0;
    int *previous_level = &zero;

    int is_increasing = report[0] - report[1] < 0;

    for (int i = 0; i < length; i++) {
        if (*previous_level == 0) {
            previous_level = &report[i];
            continue;
        }

        // Level stayed the same
        if (*previous_level == report[i]) {
            return 0;
        }

        // Level increase error
        if (is_increasing != (*previous_level - report[i] < 0)) {
            return 0;
        }

        // Level diff too high
        if (abs(*previous_level - report[i]) > 3) {
            return 0;
        }

        previous_level = &report[i];
    }

    return 1;
}

int calculate_total_safe_reports(int reports[][MAX_LEVELS], int counts[]) {
    int total_safe_reports = 0;

    for (int i = 0; i < MAX_REPORTS; i++) {
        total_safe_reports += is_report_safe(reports[i], counts[i]);
    }

    return total_safe_reports;
}

int calculate_total_safe_reports_dampener(int reports[][MAX_LEVELS], int counts[]) {
    int total_safe_reports = 0;

    for (int i = 0; i < MAX_REPORTS; i++) {
        for (int j = 0; j < counts[i]; j++) {
            int temp_list[counts[i - 1]];

            for (int k = 0; k < j; k++) {
                temp_list[k] = reports[i][k];
            }

            for (int k = j + 1; k < counts[i]; k++) {
                temp_list[k - 1] = reports[i][k];
            }

            if (is_report_safe(temp_list, counts[i] - 1) == 1) {
                total_safe_reports++;
                break;
            }
        }
    }

    return total_safe_reports;
}

int main() {
    int reports[MAX_REPORTS][MAX_LEVELS] = {0};
    int rows_loaded = 0;
    int counts[MAX_REPORTS] = {0};

    load_inputs(reports, &rows_loaded, counts);

    if (rows_loaded < 0) {
        fprintf(stderr, "Failed to load data from file.\n");
        return 1;
    }

    int total_safe_reports = calculate_total_safe_reports(reports, counts);
    printf("There are %d Safe Reports\n", total_safe_reports);

    int total_safe_reports_dampener = calculate_total_safe_reports_dampener(reports, counts);
    printf("There are  %d Safe Reports using the Problem Dampener\n", total_safe_reports_dampener);

    return 0;
}
