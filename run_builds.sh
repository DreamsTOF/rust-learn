#!/bin/bash
RESULTS="/c/code/testruetlearn/build_summary.txt"
TARGET_DIR="/c/code/testruetlearn/target_batch"
rm -f "$RESULTS"

echo "==========================================" >> "$RESULTS"
echo "  Leptos Build Results Summary" >> "$RESULTS"
echo "==========================================" >> "$RESULTS"
echo "" >> "$RESULTS"

for dir in \
    "e361_focus_management" \
    "e361_focus_management_answer" \
    "e362_notification_system" \
    "e362_notification_system_answer" \
    "e363_modal_component" \
    "e363_modal_component_answer" \
    "e364_tooltip_component" \
    "e364_tooltip_component_answer" \
    "e365_cascade_selector" \
    "e365_cascade_selector_answer"
do
    PROJECT_PATH="/c/code/testruetlearn/leptos-learn/08_advanced/$dir"
    LOGFILE="/c/code/testruetlearn/build_${dir}.log"
    
    echo "Building: $dir" >> "$RESULTS"
    echo "----------------------------------------" >> "$RESULTS"
    
    cd "$PROJECT_PATH"
    CARGO_TARGET_DIR="$TARGET_DIR" NO_COLOR=true trunk build > "$LOGFILE" 2>&1
    EXIT_CODE=$?
    
    echo "Exit code: $EXIT_CODE" >> "$RESULTS"
    if [ $EXIT_CODE -eq 0 ]; then
        echo "Result: SUCCESS" >> "$RESULTS"
    else
        echo "Result: FAILED" >> "$RESULTS"
        # Extract the actual error from the end of the log
        tail -20 "$LOGFILE" >> "$RESULTS"
    fi
    echo "" >> "$RESULTS"
done

echo "==========================================" >> "$RESULTS"
echo "Build complete!" >> "$RESULTS"
