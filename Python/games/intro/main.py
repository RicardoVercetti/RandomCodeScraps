"""Breakout Game"""
from dataclasses import dataclass
import pygame

# making Breakout game with just trial and error

WIDTH = 1280
HEIGHT = 720

pygame.init()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption("Pygame")
clock = pygame.time.Clock()
RUNNING = True

# slate position
@dataclass
class Slate:
    left_position: float
    right_position: float
    length_of_slab: float
    thickness: float

length = 180
slate = Slate(WIDTH/2 - length/2, HEIGHT - 40, length, 20)

# slabs positioned at the top

# spawn the ball with a momentum upwards,
# when the game starts, the ball have to be frozen util any button press happens


# bounce logic:
# 1. if the ball hits the left or right sides, bounces normal - just change the horizontal direction
# 2. if it hits the corners of a slab, the bounce kinda skids to the direction that its going towards
# 3. when it hits the base slate
#   - if the slate doesn't move when the ball hits the slate - direction changes normal
#   - if the slate moves in the same horizontal direction that the ball moves, the bounce skids more
#   - if slate moves in the opposite direction, the bounce is less or the direction is reversed horizontally

while RUNNING:
    for event in pygame.event.get():
        if event.type == pygame.constants.QUIT:
            RUNNING = False
    
    screen.fill("purple")

    # get the keyboard inputs to move the slate
    key_pressed = pygame.key.get_pressed()
    if key_pressed[pygame.K_LEFT]:
        if slate.left_position > 0:
            slate.left_position -= 13

    if key_pressed[pygame.K_RIGHT]:
        if slate.left_position + slate.length_of_slab < WIDTH:
            slate.left_position += 13

    # fill screen
    screen.fill("blue")

    # draw the base slate
    rect = pygame.Rect(slate.left_position, slate.right_position, slate.length_of_slab, slate.thickness)
    pygame.draw.rect(screen, "yellow", rect)


    pygame.display.flip()
    clock.tick(60)


pygame.quit()
