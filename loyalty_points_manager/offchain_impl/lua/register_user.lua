-- register_user.lua
-- KEYS[1]: Users table ie lpm:pubkeys
-- ARGV[1]: User Pubkey

-- lpm:level:<val> gives the hashes for each level from 19 to 0 assuming MAX_DEPTH_SIZE = 20.
-- lpm:pubkeys is a self constructed index having a one to one
-- mapping from pubkey to lpm_<levelno>_<index>

local v = redis.call("HGET", KEYS[1], ARGV[1])
if v == true then
    -- pubkey exists in the redis db.
    return 0
end

local index = tonumber(v)
-- result has first member as sibling node.
local result = {}
if index % 2 == 0 then
    table.insert(result,index + 1)
else
    table.insert(result,index - 1)
end

-- it has indices of parent nodes of levels above.
local val = index
while val > 1 do
    val = math.floor(val / 2)
    table.insert(result, val)
end


-- fetch hashes from all levels. 
local level_no = 19 -- TODO:change this to MAX_DEPTH_SIZE
local hashes = {}
for index_no in result do
    local val = redis.call("HGET","lpm:level:"+level_no,index_no)
    table.insert(hashes,val)
end


return hashes
