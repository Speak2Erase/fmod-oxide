// Copyright (FMOD_CHANNELCONTROL* channelcontrol, c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#ifndef FMOD_CHANNEL_CONTROL_H
#define FMOD_CHANNEL_CONTROL_H

#include <fmod.h>
#include <stdbool.h>

// interface with the c++ fmod bindings that adds c function definitions of
// ChannelControl functions

// why use bool instead of FMOD_BOOL? for starters, this is not an official
// header. the C++ api uses bool, and casting between FMOD_BOOL and bool is not
// possible. so we just use bool.

#ifdef __cplusplus
extern "C" {
#endif

// these are not wrappers to fmod code specifically, they're just functions to
// check that you can just perform pointer casting to get the correct type
//
// (they do also cast the pointer, so you could use that too if you are using
// these raw bindings for whatever reason)
FMOD_CHANNELCONTROL *FMOD_Channel_CastToControl(FMOD_CHANNEL *channel);
FMOD_CHANNELCONTROL *FMOD_ChannelGroup_CastToControl(FMOD_CHANNELGROUP *group);

FMOD_RESULT
FMOD_ChannelControl_GetSystemObject(FMOD_CHANNELCONTROL *channelcontrol,
                                    FMOD_SYSTEM **system);

// General control functionality for Channels and ChannelGroups.
FMOD_RESULT FMOD_ChannelControl_Stop(FMOD_CHANNELCONTROL *channelcontrol);
FMOD_RESULT
FMOD_ChannelControl_SetPaused(FMOD_CHANNELCONTROL *channelcontrol, bool paused);
FMOD_RESULT FMOD_ChannelControl_GetPaused(FMOD_CHANNELCONTROL *channelcontrol,
                                          bool *paused);
FMOD_RESULT FMOD_ChannelControl_SetVolume(FMOD_CHANNELCONTROL *channelcontrol,
                                          float volume);
FMOD_RESULT FMOD_ChannelControl_GetVolume(FMOD_CHANNELCONTROL *channelcontrol,
                                          float *volume);
FMOD_RESULT
FMOD_ChannelControl_SetVolumeRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                  bool ramp);
FMOD_RESULT
FMOD_ChannelControl_GetVolumeRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                  bool *ramp);
FMOD_RESULT
FMOD_ChannelControl_GetAudibility(FMOD_CHANNELCONTROL *channelcontrol,
                                  float *audibility);
FMOD_RESULT
FMOD_ChannelControl_SetPitch(FMOD_CHANNELCONTROL *channelcontrol, float pitch);
FMOD_RESULT
FMOD_ChannelControl_GetPitch(FMOD_CHANNELCONTROL *channelcontrol, float *pitch);
FMOD_RESULT
FMOD_ChannelControl_SetMute(FMOD_CHANNELCONTROL *channelcontrol, bool mute);
FMOD_RESULT
FMOD_ChannelControl_GetMute(FMOD_CHANNELCONTROL *channelcontrol, bool *mute);
FMOD_RESULT
FMOD_ChannelControl_SetReverbProperties(FMOD_CHANNELCONTROL *channelcontrol,
                                        int instance, float wet);
FMOD_RESULT
FMOD_ChannelControl_GetReverbProperties(FMOD_CHANNELCONTROL *channelcontrol,
                                        int instance, float *wet);
FMOD_RESULT
FMOD_ChannelControl_SetLowPassGain(FMOD_CHANNELCONTROL *channelcontrol,
                                   float gain);
FMOD_RESULT
FMOD_ChannelControl_GetLowPassGain(FMOD_CHANNELCONTROL *channelcontrol,
                                   float *gain);
FMOD_RESULT FMOD_ChannelControl_SetMode(FMOD_CHANNELCONTROL *channelcontrol,
                                        FMOD_MODE mode);
FMOD_RESULT FMOD_ChannelControl_GetMode(FMOD_CHANNELCONTROL *channelcontrol,
                                        FMOD_MODE *mode);
FMOD_RESULT
FMOD_ChannelControl_SetCallback(FMOD_CHANNELCONTROL *channelcontrol,
                                FMOD_CHANNELCONTROL_CALLBACK callback);
FMOD_RESULT FMOD_ChannelControl_IsPlaying(FMOD_CHANNELCONTROL *channelcontrol,
                                          bool *isplaying);

// Panning and level adjustment.
// Note all 'set' functions alter a final matrix, this is why the only get
// function is getMixMatrix, to avoid other get functions returning
// incorrect/obsolete values.
FMOD_RESULT
FMOD_ChannelControl_SetPan(FMOD_CHANNELCONTROL *channelcontrol, float pan);
FMOD_RESULT FMOD_ChannelControl_SetMixLevelsOutput(
    FMOD_CHANNELCONTROL *channelcontrol, float frontleft, float frontright,
    float center, float lfe, float surroundleft, float surroundright,
    float backleft, float backright);
FMOD_RESULT
FMOD_ChannelControl_SetMixLevelsInput(FMOD_CHANNELCONTROL *channelcontrol,
                                      float *levels, int numlevels);
FMOD_RESULT
FMOD_ChannelControl_SetMixMatrix(FMOD_CHANNELCONTROL *channelcontrol,
                                 float *matrix, int outchannels, int inchannels,
                                 int inchannel_hop);
FMOD_RESULT
FMOD_ChannelControl_GetMixMatrix(FMOD_CHANNELCONTROL *channelcontrol,
                                 float *matrix, int *outchannels,
                                 int *inchannels, int inchannel_hop);

// Clock based functionality.
FMOD_RESULT FMOD_ChannelControl_GetDSPClock(FMOD_CHANNELCONTROL *channelcontrol,
                                            unsigned long long *dspclock,
                                            unsigned long long *parentclock);
FMOD_RESULT FMOD_ChannelControl_SetDelay(FMOD_CHANNELCONTROL *channelcontrol,
                                         unsigned long long dspclock_start,
                                         unsigned long long dspclock_end,
                                         bool stopchannels);
FMOD_RESULT FMOD_ChannelControl_GetDelay(FMOD_CHANNELCONTROL *channelcontrol,
                                         unsigned long long *dspclock_start,
                                         unsigned long long *dspclock_end,
                                         bool *stopchannels);
FMOD_RESULT
FMOD_ChannelControl_AddFadePoint(FMOD_CHANNELCONTROL *channelcontrol,
                                 unsigned long long dspclock, float volume);
FMOD_RESULT
FMOD_ChannelControl_SetFadePointRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                     unsigned long long dspclock, float volume);
FMOD_RESULT
FMOD_ChannelControl_RemoveFadePoints(FMOD_CHANNELCONTROL *channelcontrol,
                                     unsigned long long dspclock_start,
                                     unsigned long long dspclock_end);
FMOD_RESULT FMOD_ChannelControl_GetFadePoints(
    FMOD_CHANNELCONTROL *channelcontrol, unsigned int *numpoints,
    unsigned long long *point_dspclock, float *point_volume);

// DSP effects.
FMOD_RESULT FMOD_ChannelControl_GetDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                       int index, FMOD_DSP **dsp);
FMOD_RESULT FMOD_ChannelControl_AddDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                       int index, FMOD_DSP *dsp);
FMOD_RESULT FMOD_ChannelControl_RemoveDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                          FMOD_DSP *dsp);
FMOD_RESULT FMOD_ChannelControl_GetNumDSPs(FMOD_CHANNELCONTROL *channelcontrol,
                                           int *numdsps);
FMOD_RESULT FMOD_ChannelControl_SetDSPIndex(FMOD_CHANNELCONTROL *channelcontrol,
                                            FMOD_DSP *dsp, int index);
FMOD_RESULT FMOD_ChannelControl_GetDSPIndex(FMOD_CHANNELCONTROL *channelcontrol,
                                            FMOD_DSP *dsp, int *index);

// 3D functionality.
FMOD_RESULT
FMOD_ChannelControl_Set3DAttributes(FMOD_CHANNELCONTROL *channelcontrol,
                                    const FMOD_VECTOR *pos,
                                    const FMOD_VECTOR *vel);
FMOD_RESULT
FMOD_ChannelControl_Get3DAttributes(FMOD_CHANNELCONTROL *channelcontrol,
                                    FMOD_VECTOR *pos, FMOD_VECTOR *vel);
FMOD_RESULT
FMOD_ChannelControl_Set3DMinMaxDistance(FMOD_CHANNELCONTROL *channelcontrol,
                                        float mindistance, float maxdistance);
FMOD_RESULT
FMOD_ChannelControl_Get3DMinMaxDistance(FMOD_CHANNELCONTROL *channelcontrol,
                                        float *mindistance, float *maxdistance);
FMOD_RESULT FMOD_ChannelControl_Set3DConeSettings(
    FMOD_CHANNELCONTROL *channelcontrol, float insideconeangle,
    float outsideconeangle, float outsidevolume);
FMOD_RESULT FMOD_ChannelControl_Get3DConeSettings(
    FMOD_CHANNELCONTROL *channelcontrol, float *insideconeangle,
    float *outsideconeangle, float *outsidevolume);
FMOD_RESULT
FMOD_ChannelControl_Set3DConeOrientation(FMOD_CHANNELCONTROL *channelcontrol,
                                         FMOD_VECTOR *orientation);
FMOD_RESULT
FMOD_ChannelControl_Get3DConeOrientation(FMOD_CHANNELCONTROL *channelcontrol,
                                         FMOD_VECTOR *orientation);
FMOD_RESULT
FMOD_ChannelControl_Set3DCustomRolloff(FMOD_CHANNELCONTROL *channelcontrol,
                                       FMOD_VECTOR *points, int numpoints);
FMOD_RESULT
FMOD_ChannelControl_Get3DCustomRolloff(FMOD_CHANNELCONTROL *channelcontrol,
                                       FMOD_VECTOR **points, int *numpoints);
FMOD_RESULT
FMOD_ChannelControl_Set3DOcclusion(FMOD_CHANNELCONTROL *channelcontrol,
                                   float directocclusion,
                                   float reverbocclusion);
FMOD_RESULT
FMOD_ChannelControl_Get3DOcclusion(FMOD_CHANNELCONTROL *channelcontrol,
                                   float *directocclusion,
                                   float *reverbocclusion);
FMOD_RESULT FMOD_ChannelControl_Set3DSpread(FMOD_CHANNELCONTROL *channelcontrol,
                                            float angle);
FMOD_RESULT FMOD_ChannelControl_Get3DSpread(FMOD_CHANNELCONTROL *channelcontrol,
                                            float *angle);
FMOD_RESULT FMOD_ChannelControl_Set3DLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                           float level);
FMOD_RESULT FMOD_ChannelControl_Get3DLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                           float *level);
FMOD_RESULT
FMOD_ChannelControl_Set3DDopplerLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                      float level);
FMOD_RESULT
FMOD_ChannelControl_Get3DDopplerLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                      float *level);
FMOD_RESULT
FMOD_ChannelControl_Set3DDistanceFilter(FMOD_CHANNELCONTROL *channelcontrol,
                                        bool custom, float customLevel,
                                        float centerFreq);
FMOD_RESULT
FMOD_ChannelControl_Get3DDistanceFilter(FMOD_CHANNELCONTROL *channelcontrol,
                                        bool *custom, float *customLevel,
                                        float *centerFreq);

// Userdata set/get.
FMOD_RESULT FMOD_ChannelControl_SetUserData(FMOD_CHANNELCONTROL *channelcontrol,
                                            void *userdata);
FMOD_RESULT FMOD_ChannelControl_GetUserData(FMOD_CHANNELCONTROL *channelcontrol,
                                            void **userdata);

#ifdef __cplusplus
}
#endif

#endif