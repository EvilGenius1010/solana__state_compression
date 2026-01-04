-- register_user.lua
-- KEYS[1]: Hash table (lpm:19)
-- ARGV[1]: User Pubkey

local user_map_key = KEYS[1]
local pubkey = ARGV[1]

-- 1. Check if user already exists
local existing_index = redis.call('HGET', pubkey)
if existing_index then
    -- Return special table format: {status, index}
    return {0, existing_index} -- 1 = "user exists"
end

-- 2. User doesn't exist - allocate new index
local new_index = redis.call('INCR', counter_key) - 1 -- 0-based indexing

-- 3. Save pubkey -> index mapping
redis.call('HSET', user_map_key, pubkey, new_index)

-- 4. Return status + new index
return {0, new_index} -- 0 = "new user created"
