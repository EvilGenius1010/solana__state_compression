-- initialize_tree.lua
-- KEYS[1]: Tree ID Prefix (e.g., "tree:1")
-- ARGV[1]: Max Depth (e.g., 20)

local tree_prefix = KEYS[1]
local max_depth = tonumber(ARGV[1])


-- Initialize counter for next index
local counter_key = tree_prefix .. ":next_index"
redis.call('SET', counter_key, 0)
-- Create empty hashes for each level
for level = 0, max_depth - 1 do
    local level_key = tree_prefix .. ":level:" .. level
    -- Redis doesn't need explicit "create empty hash"
    -- But we can set a dummy field to ensure key exists, then delete it
    redis.call('HSET', level_key, 'max_capacity',2^level)
    redis.call('SET', counter_key, 1)
    -- redis.call('HDEL', level_key, '__init__')
end

-- Initialize secondary index (mapping from index to pubkey)
-- local user_map_key = tree_prefix .. ":users"
-- redis.call('HSET', user_map_key, '__init__', '1')
-- redis.call('HDEL', user_map_key, '__init__')


return "Tree initialized with " .. max_depth .. " levels"
