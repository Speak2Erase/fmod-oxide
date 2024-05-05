// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#include "channel_control.h"
#include <fmod.hpp>
#include <stdbool.h>

using namespace FMOD;

extern "C" {

// just c++ casts to go from channel subclass to channelcontrol superclass
FMOD_CHANNELCONTROL *FMOD_Channel_CastToControl(FMOD_CHANNEL *channel) {
  Channel *c = (Channel *)channel;
  ChannelControl *cc = static_cast<ChannelControl *>(c);
  return (FMOD_CHANNELCONTROL *)cc;
}
FMOD_CHANNELCONTROL *FMOD_ChannelGroup_CastToControl(FMOD_CHANNELGROUP *group) {
  ChannelGroup *c = (ChannelGroup *)group;
  ChannelControl *cc = static_cast<ChannelControl *>(c);
  return (FMOD_CHANNELCONTROL *)cc;
}

FMOD_RESULT
FMOD_ChannelControl_GetSystemObject(FMOD_CHANNELCONTROL *channelcontrol,
                                    FMOD_SYSTEM **system) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getSystemObject((System **)system);
}

// General control functionality for Channels and ChannelGroups.
FMOD_RESULT
FMOD_ChannelControl_stop(FMOD_CHANNELCONTROL *channelcontrol) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->stop();
}
FMOD_RESULT FMOD_ChannelControl_SetPaused(FMOD_CHANNELCONTROL *channelcontrol,
                                          bool paused) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setPaused(paused);
}
FMOD_RESULT FMOD_ChannelControl_GetPaused(FMOD_CHANNELCONTROL *channelcontrol,
                                          bool *paused) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getPaused(paused);
}
FMOD_RESULT FMOD_ChannelControl_SetVolume(FMOD_CHANNELCONTROL *channelcontrol,
                                          float volume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setVolume(volume);
}
FMOD_RESULT FMOD_ChannelControl_GetVolume(FMOD_CHANNELCONTROL *channelcontrol,
                                          float *volume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getVolume(volume);
}
FMOD_RESULT
FMOD_ChannelControl_SetVolumeRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                  bool ramp) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setVolumeRamp(ramp);
}
FMOD_RESULT
FMOD_ChannelControl_GetVolumeRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                  bool *ramp) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getVolumeRamp(ramp);
}
FMOD_RESULT
FMOD_ChannelControl_GetAudibility(FMOD_CHANNELCONTROL *channelcontrol,
                                  float *audibility) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getAudibility(audibility);
}
FMOD_RESULT
FMOD_ChannelControl_SetPitch(FMOD_CHANNELCONTROL *channelcontrol, float pitch) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setPitch(pitch);
}
FMOD_RESULT FMOD_ChannelControl_GetPitch(FMOD_CHANNELCONTROL *channelcontrol,
                                         float *pitch) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getPitch(pitch);
}
FMOD_RESULT
FMOD_ChannelControl_SetMute(FMOD_CHANNELCONTROL *channelcontrol, bool mute) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setMute(mute);
}
FMOD_RESULT
FMOD_ChannelControl_GetMute(FMOD_CHANNELCONTROL *channelcontrol, bool *mute) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getMute(mute);
}
FMOD_RESULT
FMOD_ChannelControl_SetReverbProperties(FMOD_CHANNELCONTROL *channelcontrol,
                                        int instance, float wet) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setReverbProperties(instance, wet);
}
FMOD_RESULT
FMOD_ChannelControl_GetReverbProperties(FMOD_CHANNELCONTROL *channelcontrol,
                                        int instance, float *wet) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getReverbProperties(instance, wet);
}
FMOD_RESULT
FMOD_ChannelControl_SetLowPassGain(FMOD_CHANNELCONTROL *channelcontrol,
                                   float gain) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setLowPassGain(gain);
}
FMOD_RESULT
FMOD_ChannelControl_GetLowPassGain(FMOD_CHANNELCONTROL *channelcontrol,
                                   float *gain) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getLowPassGain(gain);
}
FMOD_RESULT FMOD_ChannelControl_SetMode(FMOD_CHANNELCONTROL *channelcontrol,
                                        FMOD_MODE mode) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setMode(mode);
}
FMOD_RESULT FMOD_ChannelControl_GetMode(FMOD_CHANNELCONTROL *channelcontrol,
                                        FMOD_MODE *mode) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getMode(mode);
}
FMOD_RESULT
FMOD_ChannelControl_SetCallback(FMOD_CHANNELCONTROL *channelcontrol,
                                FMOD_CHANNELCONTROL_CALLBACK callback) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setCallback(callback);
}
FMOD_RESULT FMOD_ChannelControl_IsPlaying(FMOD_CHANNELCONTROL *channelcontrol,
                                          bool *isplaying) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->isPlaying(isplaying);
}

// Panning and level adjustment.
// Note all 'set' functions alter a final matrix, this is why the only get
// function is getMixMatrix, to avoid other get functions returning
// incorrect/obsolete values.
FMOD_RESULT
FMOD_ChannelControl_SetPan(FMOD_CHANNELCONTROL *channelcontrol, float pan) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setPan(pan);
}
FMOD_RESULT FMOD_ChannelControl_SetMixLevelsOutput(
    FMOD_CHANNELCONTROL *channelcontrol, float frontleft, float frontright,
    float center, float lfe, float surroundleft, float surroundright,
    float backleft, float backright) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setMixLevelsOutput(frontleft, frontright, center, lfe, surroundleft,
                               surroundright, backleft, backright);
}
FMOD_RESULT
FMOD_ChannelControl_SetMixLevelsInput(FMOD_CHANNELCONTROL *channelcontrol,
                                      float *levels, int numlevels) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setMixLevelsInput(levels, numlevels);
}
FMOD_RESULT
FMOD_ChannelControl_SetMixMatrix(FMOD_CHANNELCONTROL *channelcontrol,
                                 float *matrix, int outchannels, int inchannels,
                                 int inchannel_hop) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setMixMatrix(matrix, outchannels, inchannels, inchannel_hop);
}
FMOD_RESULT
FMOD_ChannelControl_GetMixMatrix(FMOD_CHANNELCONTROL *channelcontrol,
                                 float *matrix, int *outchannels,
                                 int *inchannels, int inchannel_hop) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getMixMatrix(matrix, outchannels, inchannels, inchannel_hop);
}

// Clock based functionality.
FMOD_RESULT FMOD_ChannelControl_GetDSPClock(FMOD_CHANNELCONTROL *channelcontrol,
                                            unsigned long long *dspclock,
                                            unsigned long long *parentclock) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getDSPClock(dspclock, parentclock);
}
FMOD_RESULT FMOD_ChannelControl_SetDelay(FMOD_CHANNELCONTROL *channelcontrol,
                                         unsigned long long dspclock_start,
                                         unsigned long long dspclock_end,
                                         bool stopchannels) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setDelay(dspclock_start, dspclock_end, stopchannels);
}
FMOD_RESULT FMOD_ChannelControl_GetDelay(FMOD_CHANNELCONTROL *channelcontrol,
                                         unsigned long long *dspclock_start,
                                         unsigned long long *dspclock_end,
                                         bool *stopchannels) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getDelay(dspclock_start, dspclock_end, stopchannels);
}
FMOD_RESULT
FMOD_ChannelControl_AddFadePoint(FMOD_CHANNELCONTROL *channelcontrol,
                                 unsigned long long dspclock, float volume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->addFadePoint(dspclock, volume);
}
FMOD_RESULT
FMOD_ChannelControl_SetFadePointRamp(FMOD_CHANNELCONTROL *channelcontrol,
                                     unsigned long long dspclock,
                                     float volume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setFadePointRamp(dspclock, volume);
}
FMOD_RESULT
FMOD_ChannelControl_RemoveFadePoints(FMOD_CHANNELCONTROL *channelcontrol,
                                     unsigned long long dspclock_start,
                                     unsigned long long dspclock_end) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->removeFadePoints(dspclock_start, dspclock_end);
}
FMOD_RESULT FMOD_ChannelControl_GetFadePoints(
    FMOD_CHANNELCONTROL *channelcontrol, unsigned int *numpoints,
    unsigned long long *point_dspclock, float *point_volume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getFadePoints(numpoints, point_dspclock, point_volume);
}

// DSP effects.
FMOD_RESULT FMOD_ChannelControl_GetDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                       int index, FMOD_DSP **dsp) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getDSP(index, (DSP **)dsp);
}
FMOD_RESULT FMOD_ChannelControl_AddDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                       int index, FMOD_DSP *dsp) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->addDSP(index, (DSP *)dsp);
}
FMOD_RESULT FMOD_ChannelControl_RemoveDSP(FMOD_CHANNELCONTROL *channelcontrol,
                                          FMOD_DSP *dsp) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->removeDSP((DSP *)dsp);
}
FMOD_RESULT FMOD_ChannelControl_GetNumDSPs(FMOD_CHANNELCONTROL *channelcontrol,
                                           int *numdsps) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getNumDSPs(numdsps);
}
FMOD_RESULT FMOD_ChannelControl_SetDSPIndex(FMOD_CHANNELCONTROL *channelcontrol,
                                            FMOD_DSP *dsp, int index) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setDSPIndex((DSP *)dsp, index);
}
FMOD_RESULT FMOD_ChannelControl_GetDSPIndex(FMOD_CHANNELCONTROL *channelcontrol,
                                            FMOD_DSP *dsp, int *index) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getDSPIndex((DSP *)dsp, index);
}

// 3D functionality.
FMOD_RESULT
FMOD_ChannelControl_Set3DAttributes(FMOD_CHANNELCONTROL *channelcontrol,
                                    const FMOD_VECTOR *pos,
                                    const FMOD_VECTOR *vel) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DAttributes(pos, vel);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DAttributes(FMOD_CHANNELCONTROL *channelcontrol,
                                    FMOD_VECTOR *pos, FMOD_VECTOR *vel) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DAttributes(pos, vel);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DMinMaxDistance(FMOD_CHANNELCONTROL *channelcontrol,
                                        float mindistance, float maxdistance) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DMinMaxDistance(mindistance, maxdistance);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DMinMaxDistance(FMOD_CHANNELCONTROL *channelcontrol,
                                        float *mindistance,
                                        float *maxdistance) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DMinMaxDistance(mindistance, maxdistance);
}
FMOD_RESULT FMOD_ChannelControl_Set3DConeSettings(
    FMOD_CHANNELCONTROL *channelcontrol, float insideconeangle,
    float outsideconeangle, float outsidevolume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DConeSettings(insideconeangle, outsideconeangle, outsidevolume);
}
FMOD_RESULT FMOD_ChannelControl_Get3DConeSettings(
    FMOD_CHANNELCONTROL *channelcontrol, float *insideconeangle,
    float *outsideconeangle, float *outsidevolume) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DConeSettings(insideconeangle, outsideconeangle, outsidevolume);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DConeOrientation(FMOD_CHANNELCONTROL *channelcontrol,
                                         FMOD_VECTOR *orientation) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DConeOrientation(orientation);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DConeOrientation(FMOD_CHANNELCONTROL *channelcontrol,
                                         FMOD_VECTOR *orientation) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DConeOrientation(orientation);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DCustomRolloff(FMOD_CHANNELCONTROL *channelcontrol,
                                       FMOD_VECTOR *points, int numpoints) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DCustomRolloff(points, numpoints);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DCustomRolloff(FMOD_CHANNELCONTROL *channelcontrol,
                                       FMOD_VECTOR **points, int *numpoints) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DCustomRolloff(points, numpoints);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DOcclusion(FMOD_CHANNELCONTROL *channelcontrol,
                                   float directocclusion,
                                   float reverbocclusion) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DOcclusion(directocclusion, reverbocclusion);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DOcclusion(FMOD_CHANNELCONTROL *channelcontrol,
                                   float *directocclusion,
                                   float *reverbocclusion) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DOcclusion(directocclusion, reverbocclusion);
}
FMOD_RESULT FMOD_ChannelControl_Set3DSpread(FMOD_CHANNELCONTROL *channelcontrol,
                                            float angle) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DSpread(angle);
}
FMOD_RESULT FMOD_ChannelControl_Get3DSpread(FMOD_CHANNELCONTROL *channelcontrol,
                                            float *angle) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DSpread(angle);
}
FMOD_RESULT FMOD_ChannelControl_Set3DLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                           float level) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DLevel(level);
}
FMOD_RESULT FMOD_ChannelControl_Get3DLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                           float *level) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DLevel(level);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DDopplerLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                      float level) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DDopplerLevel(level);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DDopplerLevel(FMOD_CHANNELCONTROL *channelcontrol,
                                      float *level) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DDopplerLevel(level);
}
FMOD_RESULT
FMOD_ChannelControl_Set3DDistanceFilter(FMOD_CHANNELCONTROL *channelcontrol,
                                        bool custom, float customLevel,
                                        float centerFreq) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->set3DDistanceFilter(custom, customLevel, centerFreq);
}
FMOD_RESULT
FMOD_ChannelControl_Get3DDistanceFilter(FMOD_CHANNELCONTROL *channelcontrol,
                                        bool *custom, float *customLevel,
                                        float *centerFreq) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->get3DDistanceFilter(custom, customLevel, centerFreq);
}

// Userdata set/get.
FMOD_RESULT FMOD_ChannelControl_SetUserData(FMOD_CHANNELCONTROL *channelcontrol,
                                            void *userdata) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->setUserData(userdata);
}
FMOD_RESULT FMOD_ChannelControl_GetUserData(FMOD_CHANNELCONTROL *channelcontrol,
                                            void **userdata) {
  ChannelControl *c = (ChannelControl *)channelcontrol;
  return c->getUserData(userdata);
}
}
