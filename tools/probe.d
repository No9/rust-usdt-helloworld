#pragma D option quiet
myserv*:::
{
	printf("%s Fired %d\n",probefunc, arg0);

	@[pid] = count();
}
