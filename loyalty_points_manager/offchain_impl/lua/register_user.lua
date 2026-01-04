-- register_user.lua
-- KEYS[1]: User Map (tree:1:users)
-- KEYS[2]: Counter (tree:1:next_index)
-- ARGV[1]: User Pubkey

local user_map_key = KEYS[1]
local counter_key = KEYS[2]
local pubkey = ARGV[1]

-- 1. Check if user already exists
local existing_index = redis.call('HGET', user_map_key, pubkey)
if existing_index then
    -- Return special table format: {status, index}
    return {1, tonumber(existing_index)} -- 1 = "user exists"
end

-- 2. User doesn't exist - allocate new index
local new_index = redis.call('INCR', counter_key) - 1 -- 0-based indexing

-- 3. Save pubkey -> index mapping
redis.call('HSET', user_map_key, pubkey, new_index)

-- 4. Return status + new index
return {0, new_index} -- 0 = "new user created"
