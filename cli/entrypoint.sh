#!/bin/sh -l

# If args input is provided, use it directly
if [ -n "$INPUT_ARGS" ]; then
    exec luna_cli $INPUT_ARGS
fi

COMMAND="${INPUT_COMMAND:-generate}"
CMD="luna_cli $COMMAND"

if [ "$COMMAND" = "generate" ]; then
    if [ -n "$INPUT_PATH" ]; then
        CMD="$CMD $INPUT_PATH"
    fi
elif [ "$COMMAND" = "docs" ]; then
    PATH_VAL="${INPUT_PATH:-output/docs}"
    # If index path is set, we must provide path
    if [ -n "$INPUT_INDEX_PATH" ]; then
        CMD="$CMD $PATH_VAL $INPUT_INDEX_PATH"
    elif [ -n "$INPUT_PATH" ]; then
        # If only path is set
        CMD="$CMD $INPUT_PATH"
    fi
    
    if [ -n "$INPUT_PAGE_SIZE" ]; then
        CMD="$CMD --page-size $INPUT_PAGE_SIZE"
    fi
elif [ "$COMMAND" = "build" ]; then
    INDEX_PATH="${INPUT_INDEX_PATH:-output/index.json}"
    DOCS_PATH="${INPUT_PATH:-output/docs}"
    
    echo "Running: luna_cli generate $INDEX_PATH"
    luna_cli generate "$INDEX_PATH" || exit 1
    
    CMD="luna_cli docs $DOCS_PATH $INDEX_PATH"
    if [ -n "$INPUT_PAGE_SIZE" ]; then
        CMD="$CMD --page-size $INPUT_PAGE_SIZE"
    fi
fi

echo "Running: $CMD"
exec $CMD
