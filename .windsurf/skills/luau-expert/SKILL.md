---
name: luau-expert
description: Expert guidance for Luau programming. ALWAYS use this skill when writing, reviewing, or debugging Luau code—especially for Roblox game development, embedded systems, or any Luau-specific tasks.
version: 2.0
last_updated: 2026-05-02
priority: critical
activation: auto
keywords: [luau, roblox, game-development, lua, typing, performance, security]
---

# Luau Expert

Expert guidance for Luau programming with deep knowledge of its unique features, performance characteristics, and common use cases in game development (particularly Roblox) and embedded systems.

## When to Use
- Writing or reviewing Luau code
- Debugging Luau-specific issues
- Working with Roblox game scripts, ModuleScripts, or game development
- Questions about Luau syntax, typing, or performance optimization
- Implementing error handling, memory management, or security practices in Luau
- Working with client-server validation, remote events, or Luau-specific patterns
- Using Luau-specific features like compound operators (+=, -=), continue keyword, table.clone(), table.freeze(), typeof(), or the task library
- Designing data structures or algorithms in Luau
- Writing tests for Luau code
- Migrating from standard Lua to Luau
- Optimizing performance-critical game systems
- Implementing anti-cheat or security measures

## When NOT to Use
- Writing standard Lua code (without Luau-specific features) - use generic Lua resources instead
- Working with other programming languages - use language-specific skills
- General game development questions unrelated to Luau implementation - use game-dev skills
- Roblox Studio UI/UX design questions - use design-focused skills
- Roblox asset creation (models, animations, audio) - use asset-focused resources

## Core Principles

Write clear, concise Luau code following modern idiomatic patterns. Leverage Luau's gradual typing system to catch bugs early while maintaining code clarity. Use proper error handling with `pcall`/`xpcall` and the `task` library. Follow consistent naming conventions and modular code organization. Optimize for performance by utilizing Luau's specific built-in optimizations.

**Key Philosophy:** Type safety first, performance second, readability third. Luau's type system is your primary defense against bugs—use it aggressively.

## Critical Luau-Specific Features

### Type Checking
Use `--!strict` at the top of scripts. Define custom types and use type annotations:

```lua
--!strict
type UserData = {
	name: string,
	id: number,
	isAdmin: boolean?
}

local function processUser(user: UserData): string
	return user.name
end
```

### Compound Operators
Use Luau's compound assignment operators for cleaner code:

```lua
-- Instead of: count = count + 1
-- Use:
count += 1

-- String concatenation:
message ..= " appended"
```

### Continue Keyword
Use `continue` to skip loop iterations:

```lua
for i = 1, 10 do
	if i % 2 == 0 then
		continue
	end
	print(i) -- Only odd numbers
end
```

### Table Operations
Use Luau-specific table functions:

```lua
local original = {a = 1, b = 2}
local cloned = table.clone(original)
local frozen = table.freeze(cloned)

-- table.find for searching
local index = table.find(array, value)

-- table.move for efficient segment moves
table.move(source, from, to, target, toIndex)
```

### Task Library
Prefer `task` library over legacy `coroutine`:

```lua
task.spawn(function()
	-- Async work
end)

task.delay(5, function()
	-- Delayed execution
end)

-- task.wait is more accurate than wait()
task.wait(0.1) -- 0.1 seconds, respects frame timing

-- task.defer runs after current call stack completes
task.defer(function()
	-- Cleanup or follow-up work
end)
```

## Performance Optimization

**Pre-allocate tables** when size is known:
```lua
local results = table.create(expectedSize)
```

**Minimize allocations in hot loops** (e.g., `RunService.Heartbeat`). Avoid creating tables or closures inside render loops.

**Use table.concat for string building**:
```lua
local parts = {}
for i = 1, 100 do
	table.insert(parts, tostring(i))
end
local result = table.concat(parts, ", ")
```

**Avoid getfenv/setfenv** - they disable Luau's performance optimizations.

**Use typeof() for type checking** - it accurately identifies engine types like `Vector3`, `Instance`, etc.

**Cache expensive operations**:
```lua
-- Cache service references
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Players = game:GetService("Players")

-- Cache frequently accessed properties
local workspace = workspace
local runService = game:GetService("RunService")
```

## Security (Critical for Roblox)

**NEVER trust the client**. Validate all RemoteEvent/RemoteFunction arguments on the server:

```lua
local function validatePlayerData(data: {userId: number, action: string}): boolean
	return type(data) == "table"
		and type(data.userId) == "number"
		and data.userId > 0
		and type(data.action) == "string"
		and data.action:match("^[a-z_]+$") -- Whitelist allowed actions
end

RemoteEvent.OnServerEvent:Connect(function(player, data)
	if not validatePlayerData(data) then
		player:Kick("Invalid data")
		return
	end
	-- Process valid data
end)
```

**Rate limiting for RemoteEvents**:
```lua
local RATE_LIMIT = 10 -- requests per second
local playerRequests: {[Player]: {number, number}} = {}

RemoteEvent.OnServerEvent:Connect(function(player, data)
	local now = tick()
	local lastRequest = playerRequests[player]

	if lastRequest and (now - lastRequest[1]) < 1/RATE_LIMIT then
		player:Kick("Rate limit exceeded")
		return
	end

	playerRequests[player] = {now, (lastRequest and lastRequest[2] or 0) + 1}
	-- Process request
end)
```

**Server-authoritative state**:
```lua
-- Store authoritative state on server
local playerStates: {[Player]: PlayerState} = {}

-- Never accept client-provided position/health/etc
-- Always calculate on server and sync to clients
```

**Avoid loadstring()** - it's disabled for security and performance reasons.

**Sanitize user input** before displaying or storing.

## Naming Conventions

- Variables/functions: `snake_case` or `camelCase` (be consistent)
- Classes/types/modules: `PascalCase`
- Constants: `UPPERCASE_SNAKE_CASE`
- Private members: prefix with `_` (`_privateVar`)

Example:
```lua
local MAX_PLAYERS = 16
local playerCount = 0
local function _calculateDamage(damage: number): number
	return damage * 1.5
end

type PlayerData = {
	userId: number,
	username: string
}
```

## Error Handling

Use `pcall` for risky operations:
```lua
local success, result = pcall(function()
	return HttpService:GetAsync(url)
end)

if not success then
	warn("Request failed:", result)
end
```

Handle nil explicitly with optional types:
```lua
local function getName(data: {name: string?}): string
	return data.name or "Unknown"
end
```

## Code Organization

Structure modules logically:
```lua
-- Constants
local MAX_RETRIES = 3

-- Types
type Config = {
	timeout: number,
	retries: number
}

-- Helper functions
local function _validate(config: Config): boolean
	-- ...
end

-- Main API
local function process(config: Config): boolean
	-- ...
end

return {process = process}
```

## Memory Management

**Disconnect events** when objects are destroyed:

```lua
local connection
connection = event:Connect(function()
	-- ...
end)

-- Later:
connection:Disconnect()
connection = nil
```

**Use ObjectValues for object references**:
```lua
local objectValue = Instance.new("ObjectValue")
objectValue.Value = targetObject
-- Automatically nils when object is destroyed
```

**Pool reusable objects**:
```lua
local objectPool: {[string]: Instance} = {}

local function getPooledObject(name: string): Instance
	if objectPool[name] then
		return objectPool[name]
	end
	local obj = Instance.new(name)
	objectPool[name] = obj
	return obj
end
```

**Use weak tables for caching**:
```lua
local cache = setmetatable({}, {__mode = "k"})
```

## OOP with Metatables

```lua
type Player = {
	name: string,
	health: number
}

local Player = {}
Player.__index = Player

function Player.new(name: string): Player
	return setmetatable({
		name = name,
		health = 100
	}, Player)
end

function Player:takeDamage(amount: number): ()
	self.health -= amount
end

export type Player = Player
return Player
```

## Common Pitfalls

- **Missing `--!strict`**: Always enable strict mode at the top of scripts to catch type errors early
- **Using `any` type**: Avoid `any` types - they defeat the purpose of Luau's type system
- **Creating tables/closures in hot loops**: This causes garbage collection pressure and performance issues
- **Forgetting to disconnect events**: Events not disconnected when objects are destroyed cause memory leaks
- **Trusting client input**: Never trust client-side data - always validate on the server
- **Using `loadstring()`**: Disabled for security and performance reasons
- **Inconsistent naming**: Mix of snake_case and camelCase makes code harder to read
- **Missing type annotations**: Function parameters and returns should have explicit types
- **Ignoring nil handling**: Use optional types (`?`) for nullable values and handle them explicitly
- **Skipping `pcall`**: Risky operations (HTTP, data stores) should always be wrapped in error handling
- **Mutating frozen tables**: `table.freeze()` makes tables immutable - attempting to modify will error
- **Using ipairs on sparse arrays**: Use `pairs` for sparse arrays, `ipairs` only for dense arrays
- **Assuming order in non-array tables**: Luau does not guarantee iteration order for non-array tables
- **Nested pcall without proper error propagation**: Inner errors may be swallowed
- **Not handling DataStore limits**: DataStore has size limits - chunk large data
- **Ignoring replication lag**: Client-server communication has latency - account for it in game logic

## Best Practices

- **Enable strict mode**: Always start scripts with `--!strict`
- **Use type annotations**: Define custom types and annotate all function parameters/returns
- **Leverage compound operators**: Use `+=`, `-=`, `..=` for cleaner code
- **Prefer `task` library**: Use `task.spawn()`, `task.delay()` instead of legacy `coroutine` functions
- **Pre-allocate tables**: Use `table.create()` when size is known to avoid reallocations
- **String concatenation**: Use `table.concat()` for building strings from many parts
- **Use `typeof()`**: For type checking engine types like `Vector3`, `Instance`, etc.
- **Disconnect events**: Always disconnect event connections when objects are destroyed
- **Validate on server**: Never trust client input - validate all RemoteEvent/RemoteFunction arguments
- **Use weak tables**: For caching to allow garbage collection of unused entries
- **Follow naming conventions**: Be consistent with snake_case/camelCase for variables, PascalCase for types/modules
- **Modular organization**: Structure modules with constants, types, helpers, and main API sections
- **Write tests**: Test critical game logic with unit tests
- **Profile before optimizing**: Use Roblox's microprofiler to identify actual bottlenecks
- **Use CollectionService**: For tagging and managing groups of instances
- **Implement retry logic**: For DataStore and HTTP operations with exponential backoff
- **Separate concerns**: Keep UI logic separate from game logic
- **Use attributes**: For storing metadata on instances instead of separate tables

## Code Review Checklist

- [ ] `--!strict` enabled at top of file
- [ ] No `any` types unless absolutely necessary
- [ ] Type annotations on all function parameters/returns
- [ ] No table/closure creation in hot loops (RunService.Heartbeat, RenderStepped)
- [ ] Events disconnected when objects are destroyed
- [ ] Client input validated on server with type checking and whitelisting
- [ ] No `loadstring()` usage
- [ ] Consistent naming conventions throughout file
- [ ] Optional types (`?`) for nullable values with explicit nil handling
- [ ] `pcall`/`xpcall` for risky operations (HTTP, DataStore, etc.)
- [ ] Rate limiting on RemoteEvents/RemoteFunctions
- [ ] No mutation of frozen tables
- [ ] Proper error propagation in nested calls
- [ ] DataStore operations handle size limits and retry logic
- [ ] Replication lag accounted for in game logic
- [ ] Service references cached at module level
- [ ] Module exports properly typed
- [ ] No global variables (use module pattern)
- [ ] Comments only explain WHY, not WHAT
- [ ] Dead code removed
- [ ] Magic numbers extracted to named constants

## Testing Patterns

**Unit testing with TestService**:
```lua
local TestService = game:GetService("TestService")

local function testAddition()
	local result = add(2, 3)
	assert(result == 5, "2 + 3 should equal 5")
end

TestService:Run(testAddition)
```

**Mocking for isolated tests**:
```lua
local function mockDataStore()
	return {
		GetAsync = function(self, key)
			return self.mockData[key]
		end,
		SetAsync = function(self, key, value)
			self.mockData[key] = value
		end,
		mockData = {}
	}
end
```

## Debugging Techniques

**Use debug library**:
```lua
debug.info(1, "sl") -- Get source/line info
debug.profilebegin("section")
-- code
debug.profileend()
```

**Structured logging**:
```lua
local function log(message: string, level: "info" | "warn" | "error")
	print(`[{level}] {message}`)
end
```

**Conditional breakpoints**:
```lua
if DEBUG_MODE and someCondition then
	warn("Breakpoint hit")
end
```

## Migration from Lua to Luau

**Add strict mode**:
```lua
-- Old Lua
-- No type checking

-- New Luau
--!strict
local function process(data: {name: string}): string
	return data.name
end
```

**Replace coroutine with task**:
```lua
-- Old Lua
coroutine.wrap(function()
	-- async work
end)()

-- New Luau
task.spawn(function()
	-- async work
end)
```

**Add type annotations gradually**:
1. Start with function parameters
2. Add return types
3. Define custom types for complex data
4. Use generic types for reusable functions

## Roblox-Specific Patterns

**CollectionService for tagging**:
```lua
local CollectionService = game:GetService("CollectionService")

CollectionService:GetInstanceAddedSignal("Enemy"):Connect(function(enemy)
	-- Initialize enemy
end)
```

**Attribute system**:
```lua
instance:SetAttribute("Health", 100)
local health = instance:GetAttribute("Health")
```

**Signal pattern for decoupled systems**:
```lua
local Signal = {}
Signal.__index = Signal

function Signal.new()
	return setmetatable({connections = {}}, Signal)
end

function Signal:Connect(callback)
	table.insert(self.connections, callback)
end

function Signal:Fire(...)
	for _, callback in self.connections do
		task.spawn(callback, ...)
	end
end
```

## Related Workflows
- `/code-fix-loop` - For iterative code review and refactoring
- `/debug` - For systematic debugging of Luau code issues
- `/test` - For generating and running Luau tests

## Related Skills
- `@[skills/clean-code]` - General coding standards applicable to Luau
- `@[skills/error-handling]` - Error handling patterns (adapted for Luau's pcall/xpcall)
- `@[skills/performance-profiling]` - Performance optimization techniques
- `@[skills/architecture]` - System design principles for game architectures