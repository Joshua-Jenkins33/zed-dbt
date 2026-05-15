; SQL keywords
[
  (keyword_select)
  (keyword_from)
  (keyword_where)
  (keyword_join)
  (keyword_left)
  (keyword_right)
  (keyword_inner)
  (keyword_outer)
  (keyword_full)
  (keyword_cross)
  (keyword_on)
  (keyword_as)
  (keyword_with)
  (keyword_distinct)
  (keyword_group)
  (keyword_by)
  (keyword_order)
  (keyword_having)
  (keyword_limit)
  (keyword_offset)
  (keyword_union)
  (keyword_all)
  (keyword_insert)
  (keyword_into)
  (keyword_values)
  (keyword_update)
  (keyword_set)
  (keyword_delete)
  (keyword_create)
  (keyword_alter)
  (keyword_drop)
  (keyword_table)
  (keyword_view)
  (keyword_case)
  (keyword_when)
  (keyword_then)
  (keyword_else)
  (keyword_end)
  (keyword_in)
  (keyword_not)
  (keyword_and)
  (keyword_or)
  (keyword_is)
  (keyword_null)
  (keyword_true)
  (keyword_false)
  (keyword_between)
  (keyword_like)
  (keyword_ilike)
  (keyword_exists)
  (keyword_if)
  (keyword_over)
  (keyword_partition)
  (keyword_window)
  (keyword_rows)
  (keyword_range)
  (keyword_unbounded)
  (keyword_preceding)
  (keyword_following)
  (keyword_current)
  (keyword_row)
  (keyword_cast)
  (keyword_extract)
  (keyword_filter)
  (keyword_within)
  (keyword_lateral)
  (keyword_recursive)
] @keyword

; Aggregate / window functions
(keyword_count) @function.builtin
(keyword_sum) @function.builtin
(keyword_avg) @function.builtin
(keyword_min) @function.builtin
(keyword_max) @function.builtin

; Operators
(comparison_operator) @operator
(binary_expression operator: _ @operator)

; Literals
(literal) @string
(number) @number

; Identifiers
(identifier) @variable
(object_reference name: (identifier) @variable)
(field name: (identifier) @property)

; Comments
(comment) @comment
(marginalia) @comment

; Functions
(function_call name: (identifier) @function)
(invocation name: (identifier) @function)

; Types
(keyword_int) @type
(keyword_integer) @type
(keyword_float) @type
(keyword_decimal) @type
(keyword_numeric) @type
(keyword_varchar) @type
(keyword_char) @type
(keyword_text) @type
(keyword_boolean) @type
(keyword_date) @type
(keyword_timestamp) @type
(keyword_time) @type
(keyword_interval) @type
(keyword_array) @type
(keyword_struct) @type
(keyword_variant) @type
