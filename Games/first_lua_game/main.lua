debug = true

-- Collision detection taken function from http://love2d.org/wiki/BoundingBox.lua
-- Returns true if two boxes overlap, false if they don't
-- x1,y1 are the left-top coords of the first box, while w1,h1 are its width and height
-- x2,y2,w2 & h2 are the same, but for the second box
function CheckCollision(x1,y1,w1,h1, x2,y2,w2,h2)
  return x1 < x2+w2 and
         x2 < x1+w1 and
         y1 < y2+h2 and
         y2 < y1+h1
end

local isAlive = true
local score = 0

-- Timers
-- We declare these here so we don have to edit them multiple times
local canShoot = true
local canShootTimerMax = 0.1
local canShootTimer = canShootTimerMax

-- Image Storage
local bulletImg = nil

-- Entity Storage
local bullets = {} -- array of current bullets being drawn and updated

-- Time out how far apart our shots can be


-- More timers
local createEnemyTimerMax = 0.4
local createEnemyTimer = createEnemyTimerMax

local enemyImg = nil
local enemies = {}

local player = { x = 200, y = 500, speed = 150, img = nil }
function love.load(_)
	player.img = love.graphics.newImage('assets/plane_3.png')
	bulletImg = love.graphics.newImage('assets/bullet_2_blue.png')
	enemyImg = love.graphics.newImage('assets/Aircraft_06.png')
end

function love.update(dt)
	-- An easy way to exit
	if love.keyboard.isDown('escape') then
		love.event.push('quit')
	end

	if love.keyboard.isDown('left', 'a') then
		if player.x > 0 then
			player.x = player.x - (player.speed*dt)
		end
	elseif love.keyboard.isDown('right', 'd') then
		if player.x < (love.graphics.getWidth() - player.img:getWidth()) then
			player.x = player.x + (player.speed*dt)
		end
	end

	-- Time out how far apart our shots can be.
	canShootTimer = canShootTimer - (1 * dt)
	if canShootTimer < 0 then
		canShoot = true
	end

	if love.keyboard.isDown('space', 'rctrl') and canShoot then
		-- Create some bullets
		local newBullet = { x = player.x + (player.img:getWidth()/2), y = player.y, img = bulletImg }
		table.insert(bullets, newBullet)
		canShoot = false
		canShootTimer = canShootTimerMax
	end

	-- update the positions of bullets
	for i, bullet in ipairs(bullets) do
		bullet.y = bullet.y - (250 * dt)
		if bullet.y < 0 then -- remove bullets when they pass off the screen
			table.remove(bullets, i)
		end
	end

	-- Time out enemy creation
	createEnemyTimer = createEnemyTimer - (1 * dt)
	if createEnemyTimer < 0 then
		createEnemyTimer = createEnemyTimerMax

		-- Create an enemy
		local randomNumber = math.random(20, love.graphics.getWidth() - 30)
		local newEnemy = { x = randomNumber, y = -40, img = enemyImg }
		table.insert(enemies, newEnemy)
	end

	for i, enemy in ipairs(enemies) do
		enemy.y = enemy.y + (200 * dt)
		if enemy.y > love.graphics.getHeight() + 50 then
			table.remove(enemies, i)
		end
	end

	-- run our collision detection
	-- Since there will be fewer enemies on screen than bullets we'll loop them first
	-- Also, we need to see if the enemies hit our player
	for i, enemy in ipairs(enemies) do
		for j, bullet in ipairs(bullets) do
			if CheckCollision(enemy.x, enemy.y, enemy.img:getWidth(), enemy.img:getHeight(),
					bullet.x, bullet.y, bullet.img:getWidth(), bullet.img:getHeight()) then
				table.remove(bullets, j)
				table.remove(enemies, i)
				score = score + 1
			end
		end

		if CheckCollision(enemy.x, enemy.y, enemy.img:getWidth(), enemy.img:getHeight(),
				player.x, player.y, player.img:getWidth(), player.img:getHeight())
				and isAlive then
			table.remove(enemies, i)
			isAlive = false
			canShoot = false
		end
	end

	if isAlive then
		love.graphics.draw(player.img, player.x, player.y)
	else
		love.graphics.print("Press 'R' to restart", love.graphics:getWidth()/2-50,
			love.graphics:getHeight()/2-10)
	end

	if not isAlive and love.keyboard.isDown('r') then
		-- remove all our bullets and enemies from screen
		bullets = {}
		enemies = {}

		-- reset timers
		canShootTimer = canShootTimerMax
		createEnemyTimer = createEnemyTimerMax

		-- move player back to default position
		player.x = 50
		player.y = 710

		-- reset our game state
		score = 0
		isAlive = true
	end
end

function love.draw(_)
	love.graphics.draw(player.img, player.x, player.y)
	for _, bullet in ipairs(bullets) do
		love.graphics.draw(bullet.img, bullet.x, bullet.y)
	end
	for _, enemy in ipairs(enemies) do
		love.graphics.draw(enemy.img, enemy.x, enemy.y)
	end
	love.graphics.setColor(255, 255, 255)
	love.graphics.print("SCORE: " .. tostring(score), 400, 10)
end
